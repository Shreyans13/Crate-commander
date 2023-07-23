use crate::routes::error::MyError::Error;
use crate::routes::models::Email;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/trigger/otp")]
pub async fn post_trigger_otp(payload: web::Json<Email>) -> impl Responder {
    println!("{:#?}", payload);
    if payload.email.is_empty() {
        HttpResponse::Ok().json(Error {
            code: "EMAIL_NOT_PROVIDED".to_string(),
            desc: "Email not found in request".to_string(),
        })
    } else {
        HttpResponse::Ok().body("OTP Triggered ".to_owned() + &payload.email)
    }
}

#[get("/trigger/otp")]
async fn trigger_otp() -> HttpResponse {
    HttpResponse::Ok().body("OTP Triggered")
}
