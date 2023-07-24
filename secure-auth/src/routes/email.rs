use crate::database::model::{EmailTriggerResponse, EncodedObject};
use crate::mail::send_mail;
use crate::routes::models::Email;
use crate::utils::encrypt;
use crate::{database::SecuredAuthDatabase, routes::error::MyError::Error};
use actix_web::{get, post, web, HttpResponse, Responder};
use base64::Engine;

#[post("/trigger/otp")]
pub async fn post_trigger_otp(
    payload: web::Json<Email>,
    db: web::Data<SecuredAuthDatabase>,
) -> impl Responder {
    println!("{:#?}", payload);
    if payload.email.is_empty() {
        HttpResponse::Ok().json(Error {
            code: "EMAIL_NOT_PROVIDED".to_string(),
            desc: "Email not found in request".to_string(),
        })
    } else {
        let flags = otp_generator::Flags {
            digits: true,
            ..Default::default()
        };
        let otp = otp_generator::generate(6, &flags).unwrap();
        let email = &payload.email.to_owned();
        let db_otp = db.create_otp(otp.to_string(), email.to_string()).await;

        let encrypted_obj = encrypt(EncodedObject {
            message: "OTP sent to user".to_string(),
            id: db_otp.inserted_id.to_string(),
            check: email.to_string(),
        });

        match encrypted_obj {
            Ok(encoded) => {
                let encoded_string = base64::engine::general_purpose::STANDARD.encode(encoded);
                send_mail(email.to_string(), otp.to_string());
                HttpResponse::Ok().json(EmailTriggerResponse {
                    verification_key: encoded_string.to_string(),
                })
            }
            Err(_e) => HttpResponse::Ok().json(Error {
                code: "EMAIL_NOT_PROVIDED".to_string(),
                desc: "Email not found in request".to_string(),
            }),
        }
    }
}

#[get("/trigger/otp")]
async fn trigger_otp() -> HttpResponse {
    HttpResponse::Ok().body("OTP Triggered")
}
