use actix_web::{get, HttpResponse, Responder};
pub mod email;
pub mod error;
pub mod models;

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("SERVER IS UP")
}
