use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use bb8::Pool;
use bb8_tiberius::ConnectionManager;

use crate::{contexts::{jwt_session::validate_jwt, model::{ActionResult, ResultList, TableDataParams}}, services::get_data_service::GetDataService};

pub fn get_data_scope() -> Scope {
    
    web::scope("/data")
        .service(get_table_handler)
}

#[get("/get-table")]
async fn get_table_handler(params: web::Query<TableDataParams>, pool: web::Data<Pool<ConnectionManager>>, session: Option<Identity>) -> impl Responder {

    let mut result = ActionResult::default();

    match session.map(|id| id.id()) {
        None => {
            result.error = Some("Token not found".to_string());
            return HttpResponse::Unauthorized().json(result);
        },
        Some(Ok(token)) => {
            match validate_jwt(&token) {
                Ok(_) => {
                    let data: Result<ResultList, Box<dyn std::error::Error>> = GetDataService::get_table_data(params.into_inner(), pool).await;

                    match data {
                        Ok(response) => {
                            result.result = true;
                            result.message = "Retrieve data success".to_string();
                            result.data = Some(response);
                            return HttpResponse::Ok().json(result);
                        },
                        Err(e) => {
                            result.result = true;
                            result.message = "Session active".to_string();
                            result.error = Some(e.to_string());
                            return HttpResponse::InternalServerError().json(result);
                        },
                        
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