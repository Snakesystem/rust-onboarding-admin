use std::collections::HashMap;
use actix_identity::Identity;
use actix_web::{get, web::{self}, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde_json::json;

use crate::{
    contexts::{jwt_session::validate_jwt, model::{ActionResult, ListData}}, 
    services::{generic_service::GenericService, option_service::OptionService}
};

pub fn option_scope() -> Scope {
    web::scope("/option")
        .service(get_nationality)
        .service(get_city)
        .service(get_district)
        .service(get_sub_district)
        .service(get_sales)
        .service(get_income)
        .service(get_education)
        .service(get_maritalstatus)
        .service(get_gender)
        .service(get_religion)
        .service(get_fund_source)
        .service(get_residence_status)
        .service(get_beneficiary)
        .service(get_investment_objective)
        .service(get_risk)
        .service(get_question_npwp)
        .service(get_occupation)
        .service(get_position)
        .service(get_bank)
        .service(get_nature_bussiness)
        .service(get_spouse_relationship)
        .service(get_spouse_occupation)
        .service(get_spouse_position)
        .service(get_spouse_nature_bussiness)
        .service(get_bank_rdn)
        .service(get_category)
}

#[get("/nationality")]
pub async fn get_nationality(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_nationality(pool).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/city/{city}")]
pub async fn get_city(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>, city: web::Path<String>) -> impl Responder {

    let params: String = city.into_inner();

    // Cek apakah params kosong atau hanya spasi
    if params.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "result": false,
            "message": "Bad Request",
            "error": "City is empty"
        }));
    }

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_city(pool, params.to_string()).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/district/{city_id}")]
pub async fn get_district(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>, city_id: web::Path<String>) -> impl Responder {

    let params: i32 = match GenericService::parse_param(&city_id.into_inner()) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let mut result: ActionResult<Vec<HashMap<String, String>>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<HashMap<String, String>>, _> = OptionService::get_district(pool, params).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/subdistrict/{district}")]
pub async fn get_sub_district(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>, district: web::Path<String>) -> impl Responder {

    let params: String = district.into_inner();

    // Cek apakah params kosong atau hanya spasi
    if params.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "result": false,
            "message": "Bad Request",
            "error": "District is empty"
        }));
    }

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_sub_district(pool, params.to_string()).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/sales")]
pub async fn get_sales(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let result: ActionResult<Vec<ListData>, _> = OptionService::get_sales(pool).await;

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
}

#[get("/bank")]
pub async fn get_bank(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_bank(pool).await;

                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/npwp")]
pub async fn get_question_npwp(session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_question_npwp().await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
                    result.error = data.error;

                    HttpResponse::Ok().json(result)
                    
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

#[get("/income")]
pub async fn get_income(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "IncomePerAnnum".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/education")]
pub async fn get_education(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "Educational".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/maritalstatus")]
pub async fn get_maritalstatus(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "MaritalStatus".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/gender")]
pub async fn get_gender(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "Sex".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/religion")]
pub async fn get_religion(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "Religion".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/fundsource")]
pub async fn get_fund_source(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "FundSource".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/residencestatus")]
pub async fn get_residence_status(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "ResidencyStatus".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/beneficiary")]
pub async fn get_beneficiary(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "AssetOwner".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/investmentobjective")]
pub async fn get_investment_objective(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "InvestmentObjectives".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/risk")]
pub async fn get_risk(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "Risk".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/occupation")]
pub async fn get_occupation(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_occupation(pool).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/position/{occupation_id}")]
pub async fn get_position(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>, occupation_id: web::Path<String>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    let params: i32 = match GenericService::parse_param(&occupation_id.into_inner()) {
        Ok(value) => value,
        Err(response) => return response,
    };

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_position(pool, params).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/naturebusiness/{occupation_id}/{position_id}")]
pub async fn get_nature_bussiness(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>, params: web::Path<(String, String)>) -> impl Responder {

    let mut result = ActionResult::default();

    let (occupation_id, position_id) = params.into_inner();

    let params1: i32 = match GenericService::parse_param(&occupation_id) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let params2: i32 = match GenericService::parse_param(&position_id) {
        Ok(value) => value,
        Err(response) => return response,
    };

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_nature_bussiness(pool, params1, params2).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/spouse-relationship")]
pub async fn get_spouse_relationship(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_api_lookup_data(pool, "SpouseRelationship".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/spouse-occupation")]
pub async fn get_spouse_occupation(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_api_lookup_data(pool, "SpouseOccupation".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/spouse-position")]
pub async fn get_spouse_position(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "Position".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/spouse-naturebusiness")]
pub async fn get_spouse_nature_bussiness(pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result: ActionResult<Vec<ListData>, _> = ActionResult::default();

    match session.map(|id: Identity| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {

                    let data: ActionResult<Vec<ListData>, _> = OptionService::get_lookup_data(pool, "NatureOfBusiness".to_string()).await;
                    
                    result.result = data.result;
                    result.message = data.message;
                    result.data = data.data;
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

#[get("/bank-rdn")]
pub async fn get_bank_rdn(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let result: ActionResult<Vec<ListData>, _> = OptionService::get_api_lookup_data(pool, "QuestionRDN".to_string()).await;

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
}

#[get("/category")]
pub async fn get_category(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let result: ActionResult<Vec<ListData>, _> = OptionService::get_api_lookup_data(pool, "ClientNCategory".to_string()).await;

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
    
}
