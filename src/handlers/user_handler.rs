use std::collections::HashMap;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use validator::{Validate, ValidationError};

use crate::{
    contexts::{
        jwt_session::validate_jwt, 
        model::{ActionResult, DataBankRequest, DataBeneficiaryRequest, DataPekerjaanRequest, DataPendukungRequest, DataPribadiRequest, UserInfo}}, 
    services::{user_service::UserService, validation_service::validator::format_validation_errors, file_service::FileService}
};

pub fn user_scope() -> Scope {
    
    web::scope("/user")
        .service(data_pribadi)
        .service(data_bank)
        .service(data_pekerjaan)
        .service(data_pendukung)
        .service(get_user_info)
}

#[get("/userinfo")]
pub async fn get_user_info(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

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
                            HttpResponse::Ok().json(response)
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

#[post("/data-pribadi")]
async fn data_pribadi(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<DataPribadiRequest>, session: Option<Identity>) -> impl Responder {

    if let Err(errors) = request.validate() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&errors);
        
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };

        return HttpResponse::BadRequest().json(result);
    }

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();

    let email = request.email.as_deref().ok_or_else(|| "Email is required".to_string());

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let mut request: DataPribadiRequest = request.clone(); // Ubah menjadi mutable
                    match email {
                        Ok(email) => {
                            match FileService::save_base64_image(email, &request.idcard_file, "KTP") {
                                Ok(saved_path) => request.idcard_file = saved_path,
                                Err(err) => {
                                    result.error = Some(err.to_string());
                                    return HttpResponse::InternalServerError().json(result);
                                },
                            }
                            match FileService::save_base64_image(email, &request.selfie_file, "Selfie") {
                                Ok(saved_path) => request.selfie_file = saved_path,
                                Err(err) => {
                                    result.error = Some(err.to_string());
                                    return HttpResponse::InternalServerError().json(result);
                                },
                            }
                            match FileService::save_base64_image(email, &request.signature_file, "Signature") {
                                Ok(saved_path) => request.signature_file = saved_path,
                                Err(err) => {
                                    result.error = Some(err.to_string());
                                    return HttpResponse::InternalServerError().json(result);
                                },
                            }
                        },
                        Err(err) => {
                            result.error = Some(err.to_string());
                            return HttpResponse::BadRequest().json(result);
                        },
                    }

                    let response: ActionResult<HashMap<String, String>, String> = UserService::save_data_pribadi(pool, request, claims).await;

                    result.result = response.result;
                    result.message = response.message;
                    result.data = response.data;
                    result.error = response.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
                        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
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

#[post("/data-bank")]
async fn data_bank(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<DataBankRequest>, session: Option<Identity>) -> impl Responder {

    if let Err(errors) = request.validate() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&errors);
        
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };

        return HttpResponse::BadRequest().json(result);
    }

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let response: ActionResult<HashMap<String, String>, String> = UserService::save_data_bank(pool, request.into_inner(), claims).await;

                    result.result = response.result;
                    result.message = response.message;
                    result.data = response.data;
                    result.error = response.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
                        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
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

#[post("/data-pekerjaan")]
async fn data_pekerjaan(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<DataPekerjaanRequest>, session: Option<Identity>) -> impl Responder {

    if let Err(errors) = request.validate() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&errors);
        
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };

        return HttpResponse::BadRequest().json(result);
    }

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let mut request: DataPekerjaanRequest = request.clone(); // Ubah menjadi mutable

                    match FileService::save_base64_image(&claims.email, &request.npwp_file, "NPWP") {
                        Ok(saved_path) => request.npwp_file = saved_path,
                        Err(err) => {
                            result.error = Some(err.to_string());
                            return HttpResponse::InternalServerError().json(result);
                        },
                    }

                    let response: ActionResult<HashMap<String, String>, String> = UserService::save_data_pekerjaan(pool, request, claims).await;

                    result.result = response.result;
                    result.message = response.message;
                    result.data = response.data;
                    result.error = response.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
                        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
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

#[post("/data-pendukung")]
async fn data_pendukung(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<DataPendukungRequest>, session: Option<Identity>) -> impl Responder {

    if let Err(errors) = request.validate() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&errors);
        
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };

        return HttpResponse::BadRequest().json(result);
    }

    let mut validation_errors = validator::ValidationErrors::new();

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();

    if request.question_1 && request.question_1text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_1text", error);
    }

    if request.question_2 && request.question_2text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_2text", error);
    }

    if request.question_3 && request.question_3text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_3text", error);
    }

    if request.question_4 && request.question_4text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_4text", error);
    }

    if request.question_5 && request.question_5text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_5text", error);
    }

    if request.question_6 && request.question_6text.as_deref().unwrap_or("").trim().is_empty() {
        let error = ValidationError::new("required");
        validation_errors.add("question_6text", error);
    }

    if !validation_errors.is_empty() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&validation_errors);
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };
        return HttpResponse::BadRequest().json(result);
    }

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let response: ActionResult<HashMap<String, String>, String> = UserService::save_data_pendukung(pool, request.into_inner(), claims).await;

                    result.result = response.result;
                    result.message = response.message;
                    result.data = response.data;
                    result.error = response.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
                        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
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

#[post("/beneficiary-owner")]
async fn data_beneficiary(pool: web::Data<Pool<ConnectionManager>>, request: web::Json<DataBeneficiaryRequest>, session: Option<Identity>) -> impl Responder {

    if let Err(errors) = request.validate() {
        let formatted_errors: HashMap<String, String> = format_validation_errors(&errors);
        
        let result: ActionResult<HashMap<String, String>, _> = ActionResult {
            result: false,
            message: "Validation failed".to_string(),
            data: None,
            error: Some(formatted_errors),
        };

        return HttpResponse::BadRequest().json(result);
    }

    let mut result: ActionResult<HashMap<String, String>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(claims) => {
                    let response: ActionResult<HashMap<String, String>, String> = UserService::save_data_beneficiary(pool, request.into_inner(), claims).await;

                    result.result = response.result;
                    result.message = response.message;
                    result.data = response.data;
                    result.error = response.error;

                    match result {
                        response if response.error.is_some() => {
                            HttpResponse::InternalServerError().json(response)
                        }, 
                        response if response.result => HttpResponse::Ok().json(response), // Jika berhasil, HTTP 200
                        response => HttpResponse::BadRequest().json(response), // Jika gagal, HTTP 400
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

