use actix_web::{ get,  web::{self, Json}, HttpResponse, HttpServer, Responder};
use crate::routes::logging;

#[get("/")]
pub async fn index() -> impl Responder {
    logging("GET /");
    HttpResponse::Ok().body("Hello world!|||")
}
