use actix_web::{web, HttpResponse, Responder, post};
use mongodb::Client;

use crate::AppState;
use crate::services::expense_service::add_expense as add_expense_service;
use crate::DB_NAME;

#[post("")] 
async fn add_expense(app_state: web::Data<AppState>, mongo_client: web::Data<Client>) -> impl Responder {
    let db = mongo_client.database(DB_NAME);

    let resp = match add_expense_service(db).await {
        Ok(_) => {
            HttpResponse::Ok().body(format!("TODO: add_expense()"))
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    };
    resp
}


// this function could be located in a different module
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
         // prefixes all resources and routes attached to it...
        web::scope("/expenses")
            .service(add_expense)
            
    );
}