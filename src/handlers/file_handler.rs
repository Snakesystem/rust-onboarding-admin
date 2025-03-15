use std::{collections::HashMap, env, fs::{self, File}, io::{BufWriter, Cursor, Read as _}, path::Path};
use actix_identity::Identity;
use actix_web::{get, http::header, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use handlebars::Handlebars;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use serde_json::json;

use crate::{contexts::{jwt_session::validate_jwt, model::{ActionResult, UserInfo}}, services::user_service::UserService};

pub fn file_scope() -> Scope {
    
    web::scope("/reports")
        .service(render_template)
        .service(download_file)
        .service(get_file)
        .service(download_pdf)
}

#[get("/preview-pdf")]
pub async fn render_template(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity> ) -> impl Responder {

    let mut result: ActionResult<UserInfo, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {

                    let data: ActionResult<UserInfo, _> = UserService::get_user_info(pool, claims).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = Some(data.data.unwrap());
                    result.error = data.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => {
                            // let response = response.data.unwrap();
                        
                            let template_path = "./reports/template.mustache";
                            let data = json!({
                                "url": "/api/v1",
                                "header": "Cost Insurance and Freight",
                                "data": response.data
                            });
                        
                            // Baca file template
                            let template_content = match fs::read_to_string(template_path) {
                                Ok(content) => content,
                                Err(_) => return HttpResponse::InternalServerError().body("Failed to read template file"),
                            };
                        
                            // Register template di handlebars
                            let mut handlebars = Handlebars::new();
                            handlebars.register_template_string("report", template_content).unwrap();
                        
                            let body = handlebars.render("report", &data).unwrap();
                            HttpResponse::Ok().content_type("text/html").body(body)
                        }, 
                        response => {
                            HttpResponse::BadRequest().json(response)
                        }
                    }
                    
                },
                Err(err) => {
                    result.error = Some(err.to_string());
                    return HttpResponse::Unauthorized().json(result);
                },
            }
        },
        Some(Err(_)) => {
            result.error = Some("Invalid token".to_string());
            return HttpResponse::BadRequest().json(result);
        },
    }
}

#[get("/download-pdf")]
pub async fn download_pdf(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {
    let mut result: ActionResult<UserInfo, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let data: ActionResult<UserInfo, _> = UserService::get_user_info(pool, claims).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = Some(data.data.unwrap());
                    result.error = data.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        },
                        response if response.result => {
                            let template_path = "./reports/template.mustache";
                            let json_data = json!({
                                "url": "/api/v1",
                                "header": "Cost Insurance and Freight",
                                "data": response.data
                            });

                            // Baca file template
                            let template_content = match fs::read_to_string(template_path) {
                                Ok(content) => content,
                                Err(_) => return HttpResponse::InternalServerError().body("Failed to read template file"),
                            };

                            // Render template dengan Handlebars
                            let mut handlebars = Handlebars::new();
                            handlebars.register_template_string("report", template_content).unwrap();
                            let html_content = handlebars.render("report", &json_data).unwrap();

                            // Convert HTML ke teks untuk printpdf
                            let cleaned_text = html_content
                                .replace("<br>", "\n")
                                .replace("</p>", "\n")
                                .replace("<p>", "")
                                .replace("</tr>", "\n")
                                .replace("<tr>", "")
                                .replace("</td>", " | ")
                                .replace("<td>", " ")
                                .replace("</th>", " | ")
                                .replace("<th>", " ")
                                .replace("</h1>", "\n")
                                .replace("<h1>", "\n# ")
                                .replace("</table>", "\n")
                                .replace("<table>", "\n")
                                .replace("<html>", "")
                                .replace("</html>", "")
                                .replace("<head>", "")
                                .replace("</head>", "")
                                .replace("<body>", "")
                                .replace("</body>", "");

                            // Generate PDF
                            let (doc, page, layer) = PdfDocument::new("Generated Report", Mm(210.0), Mm(297.0), "Layer 1");
                            let current_layer = doc.get_page(page).get_layer(layer);
                            let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

                            let mut y_position = 270.0;
                            for line in cleaned_text.lines() {
                                current_layer.use_text(line, 12.0, Mm(10.0), Mm(y_position), &font);
                                y_position -= 10.0;
                            }

                            // Simpan PDF ke buffer menggunakan BufWriter
                            let mut buffer = Cursor::new(Vec::new());
                            {
                                let mut writer = BufWriter::new(&mut buffer);
                                doc.save(&mut writer).unwrap();
                            } // Writer keluar dari scope, buffer bisa digunakan

                            // Ambil data PDF dari buffer
                            let pdf_data = buffer.into_inner();

                            // Response PDF
                            HttpResponse::Ok()
                                .content_type("application/pdf")
                                .append_header(("Content-Disposition", "attachment; filename=report.pdf"))
                                .body(pdf_data)
                        },
                        response => {
                            HttpResponse::BadRequest().json(response)
                        }
                    }
                },
                Err(err) => {
                    result.error = Some(err.to_string());
                    return HttpResponse::Unauthorized().json(result);
                },
            }
        },
        Some(Err(_)) => {
            result.error = Some("Invalid token".to_string());
            return HttpResponse::BadRequest().json(result);
        },
    }
}

#[get("/download/{file_path:.*}")]
pub async fn download_file(file_path: web::Path<String>) -> impl Responder {
    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();
    let path_env = env::var("PATH_ASSET").expect("PATH_ASSET harus diatur");
    let full_path = format!("{}/{}", path_env, file_path.into_inner());

    // Cek apakah file ada
    if !Path::new(&full_path).exists() {
        result.error = Some("File not found".to_string());
        return HttpResponse::NotFound().json(result);
    }

    // Buka file
    match File::open(&full_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer) {
                result.error = Some("Failed to read file".to_string());
                return HttpResponse::InternalServerError().json(result);
            }

            // Ambil nama file dari path
            let file_name = Path::new(&full_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            // Tentukan MIME type berdasarkan ekstensi
            let mime_type = if full_path.ends_with(".png") {
                "image/png"
            } else if full_path.ends_with(".jpg") || full_path.ends_with(".jpeg") {
                "image/jpeg"
            } else {
                "application/octet-stream"
            };

            // Paksa browser untuk download file
            HttpResponse::Ok()
                .insert_header((header::CONTENT_TYPE, mime_type))
                .insert_header((header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file_name)))
                .body(buffer)
        }
        Err(_) => HttpResponse::InternalServerError().json(result),
    }
}

#[get("/file/{file_path:.*}")]
pub async fn get_file(file_path: web::Path<String>) -> impl Responder {

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();
    let path_env = env::var("PATH_ASSET").expect("PATH_ASSET harus diatur");
    let full_path = format!("{}/{}", path_env, file_path.into_inner());

    // Cek apakah file ada
    if !Path::new(&full_path).exists() {
        result.error = Some("File not found".to_string());
        return HttpResponse::NotFound().json(result);
    }

    // Buka file
    match File::open(&full_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer) {
                result.error = Some("Failed to read file".to_string());
                return HttpResponse::InternalServerError().json(result);
            }

            // Tentukan MIME type berdasarkan ekstensi
            let mime_type = if full_path.ends_with(".png") {
                "image/png"
            } else if full_path.ends_with(".jpg") || full_path.ends_with(".jpeg") {
                "image/jpeg"
            } else {
                "application/octet-stream"
            };

            // Kirim file sebagai response
            HttpResponse::Ok()
                .content_type(mime_type)
                .body(buffer)
        }
        Err(_) => HttpResponse::InternalServerError().json(result),
    }
}