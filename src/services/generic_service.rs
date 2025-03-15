use actix_web::{error, web, HttpRequest, HttpResponse, Responder, Result};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde_json::json;
use sha2::{Digest, Sha256};
use tiberius::QueryStream;
use rand::{rng, Rng};

use crate::contexts::model::{ActionResult, Company};

pub struct GenericService;

impl GenericService {
    pub async fn get_company(connection: web::Data<Pool<ConnectionManager>>) -> ActionResult<Company, String> {
        let mut result = ActionResult::default();

        match connection.clone().get().await {
            Ok(mut conn) => {
                let query_result: Result<QueryStream, _> = conn.query("SELECT CompanyID, CompanyName FROM Company", &[]).await;
                match query_result {
                    Ok(rows) => {
                        if let Ok(Some(row)) = rows.into_row().await {
                            result.result = true;
                            result.message = "Company name".to_string();
                            result.data = Some(Company {
                                company_id: row.get::<&str, _>("CompanyID").map_or_else(|| "".to_string(), |s| s.to_string()),
                                company_name: row.get::<&str, _>("CompanyName").map_or_else(|| "".to_string(), |s| s.to_string()),
                            });
                            return result;
                        } else {
                            result.message = "No company found".to_string();
                            return result;
                        }
                    }
                    Err(e) => {
                        result.message = "Internal Server Error".to_string();
                        result.error = Some(e.to_string());
                        return result;
                    }
                }
            }
            Err(e) => {
                result.error = Some(e.to_string());
                return result;
            }
        }
    }

    pub async fn not_found(req: HttpRequest) -> impl Responder {
        HttpResponse::NotFound().json({
            json!({
                "result": false,
                "message": "Not Found",
                "error": format!("Url '{}' not found. Please check the URL.", req.path())
            })
        })
    }
    
    pub fn json_error_handler(err: error::JsonPayloadError, _req: &actix_web::HttpRequest) -> actix_web::Error {
        let error_message = format!("Json deserialize error: {}", err);

        let result = ActionResult::<String, _> { // <- Ubah dari ActionResult<()> ke ActionResult<String>
            result: false,
            message: "Invalid Request".to_string(),
            error: Some(error_message), // <- Sekarang cocok karena `data: Option<String>`
            data: None,
        };

        error::InternalError::from_response(err, HttpResponse::BadRequest().json(result)).into()
    }

    /// Helper untuk validasi path parameter yang harus berupa integer
    pub fn parse_param<T: std::str::FromStr>(param: &str) -> Result<T, HttpResponse> {
        param.parse::<T>().map_err(|_| {
            HttpResponse::BadRequest().json(json!({
                "result": false,
                "message": "Bad Request",
                "error": format!("Invalid parameter '{}'. Please provide a valid {}", param, std::any::type_name::<T>())
            }))
        })
    }

    pub fn random_string(length: usize) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rng();
    
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect()
    }

    pub fn random_string_by_suffix(length: usize, suffix: &str, name: &str) -> String {
        // Pastikan `name` memiliki panjang minimal 10 dengan padding
        let padded_name = format!("{:0<10}", name); // Tambah nol jika kurang dari 10 karakter
    
        // Gabungkan suffix dan padded name
        let input = format!("{}{}", suffix, padded_name);
    
        // Buat hash dari input
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash_result = hasher.finalize();
    
        // Karakter yang diizinkan (huruf A-Z dan angka 0-9)
        let alphanumeric_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    
        // Pastikan hasil string memiliki panjang yang sama persis dengan `length`
        let hashed_string: String = hash_result.iter()
            .flat_map(|&byte| vec![
                alphanumeric_chars[(byte as usize) % alphanumeric_chars.len()],
                alphanumeric_chars[((byte as usize) / 2) % alphanumeric_chars.len()]
            ])
            .take(length)
            .collect();
    
        hashed_string
    }

    pub fn get_ip_address(req: &HttpRequest) -> String {
        req.headers()
            .get("X-Forwarded-For") // Jika pakai reverse proxy seperti Nginx
            .and_then(|ip| ip.to_str().ok())
            .map_or_else(
                || req.peer_addr()
                    .map(|addr| addr.ip().to_string())
                    .unwrap_or_else(|| "Unknown IP".to_string()),
                |ip| ip.to_string(),
            )
    }
    
    // Function untuk ambil User-Agent (Device Info)
    pub fn get_device_info(req: &HttpRequest) -> String {
        req.headers()
            .get("User-Agent")
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or("Unknown Device")
            .to_string()
    }

}