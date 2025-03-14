use actix_web::{get, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;

use crate::{contexts::model::{ActionResult, Company}, services::generic_service::GenericService};

pub fn generic_scope() -> Scope {
    web::scope("/generic")
        .service(get_company)
}

#[get("/company")]
pub async fn get_company(pool: web::Data<Pool<ConnectionManager>>) -> impl Responder {

    let result: ActionResult<Company, _> = GenericService::get_company(pool).await;

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
