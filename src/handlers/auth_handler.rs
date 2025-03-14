use actix_identity::Identity;
use actix_web::{cookie::{time, Cookie, SameSite}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use crate::{
    contexts::{jwt_session::{create_jwt, validate_jwt}, 
    logger::write_log, 
    model::{ActionResult, ChangePasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest, WebUser}}, 
    services::{auth_service::AuthService, generic_service::GenericService}
};

pub fn auth_scope() -> Scope {
    
    web::scope("/auth")
        .service(login)
        .service(register)
        .service(check_session)
        .service(logout)
        .service(activation_user)
        .service(forget_password)
        .service(change_password)
}

#[post("/login")]
async fn login(req: HttpRequest, pool: web::Data<Pool<ConnectionManager>>, request: web::Json<LoginRequest>) -> impl Responder {

    let result: ActionResult<WebUser, _> = AuthService::login(pool, request.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => {
            if let Some(user) = &response.data {
                // ✅ Buat token JWT
                match create_jwt(user.clone()) {
                    Ok(token) => {
                        Identity::login(&req.extensions(), token.clone()).unwrap(); // ✅ Simpan sesi

                        // ✅ Simpan token dalam cookie
                        let cookie = Cookie::build("token", token.clone())
                            .path("/")
                            .http_only(true)
                            .same_site(SameSite::Strict)
                            .secure(false) // Ubah ke `true` jika pakai HTTPS
                            .finish();

                        return HttpResponse::Ok()
                            .cookie(cookie)
                            .json(response);
                    }
                    Err(err) => {
                        write_log("ERROR", format!("Failed to create JWT: {}", err).as_str());
                        return HttpResponse::InternalServerError().json(response);
                    }
                }
            }

            HttpResponse::BadRequest().json(response) // Jika tidak ada user, return 400
        },
        response => HttpResponse::BadRequest().json(response), // Jika gagal login, HTTP 400
    }
}

#[get("/session")]
async fn check_session(identity: Option<Identity>) -> impl Responder {

    let mut result = ActionResult::default();

    match identity.map(|id| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    result.result = true;
                    result.message = "Session active".to_string();
                    result.data = Some(claims);
                    return HttpResponse::Ok().json(result);
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
    };
}

#[post("/logout")]
async fn logout(id: Identity) -> impl Responder {
    // Hapus sesi dari actix-identity
    
    id.logout();

    // Hapus cookie dengan setting expired date
    let cookie = Cookie::build("token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(false) // Ubah ke true jika pakai HTTPS
        .max_age(time::Duration::days(-1)) // Set expired
        .finish();

    HttpResponse::Ok()
        .cookie(cookie) // Hapus cookie dengan expired
        .json(serde_json::json!({
            "result": true,
            "message": "Logout successful, cookie deleted"
        }))
}

#[post("/register")]
async fn register(req: HttpRequest, pool: web::Data<Pool<ConnectionManager>>, mut request: web::Json<RegisterRequest>) -> impl Responder {

    request.app_ipaddress = GenericService::get_ip_address(&req);

    let result: ActionResult<(), _> = AuthService::register(pool, request.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
    }
}

#[get("/activation/{otp_link}")]
async fn activation_user(pool: web::Data<Pool<ConnectionManager>>, otp_link: web::Path<String>) -> impl Responder {

    let result: ActionResult<(), _> = AuthService::activation_user(pool, otp_link.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
    }
}

#[post("/reset-password")]
async fn forget_password(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<ResetPasswordRequest>) -> impl Responder {

    let result: ActionResult<(), _> = AuthService::forget_password(pool, request.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
    }
}

#[post("/change-password")]
async fn change_password(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<ChangePasswordRequest>) -> impl Responder {

    let result: ActionResult<(), _> = AuthService::change_password(pool, request.into_inner()).await;

    match result {
        response if response.error.is_some() => {
            HttpResponse::InternalServerError().json(response)
        }, // Jika error, HTTP 500
        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
    }
}