use crate::database::model::{EmailTriggerResponse, EmailVerifyResponse, EncodedObject};
use crate::mail::send_mail;
use crate::routes::models::{Email, VerifyOtpRequest};
use crate::utils::{decrypt, encrypt};
use crate::{database::SecuredAuthDatabase, routes::error::MyError::Error};
use actix_web::{post, web, HttpResponse, Responder};
use base64::Engine;
use bson::oid::ObjectId;
use bson::{Bson, DateTime};

#[post("/trigger/otp")]
pub async fn trigger_otp(
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
        println!("otp = {:?}", otp.to_owned());
        // let otp = "111111".to_owned();
        let email = &payload.email.to_owned();
        let db_otp = db.create_otp(otp.to_string(), email.to_string()).await;
        match db_otp {
            Ok(otp_db) => {
                let uni_id = Bson::as_object_id(&otp_db.inserted_id);
                println!("uni_id  = {:?}", uni_id);
                println!("otp_db.inserted_id  = {:?}", otp_db.inserted_id);
                println!(
                    "otp_db.inserted_id  = {:?}",
                    Bson::as_object_id(&otp_db.inserted_id)
                );
                match uni_id {
                    Some(id) => {
                        // println!("otp_db.inserted_id.to_string(), = {:?}", otp_db.inserted_id);
                        let obj = EncodedObject {
                            message: "OTP sent to user".to_string(),
                            check: email.to_string(),
                            id: id.to_string(),
                        };
                        println!("\n");
                        println!(
                            "otp_db.inserted_id.to_string() = {:?}",
                            otp_db.inserted_id.to_string()
                        );
                        println!("\n");
                        let encrypted_obj = encrypt(obj);
                        match encrypted_obj {
                            Ok(encoded) => {
                                let encoded_string =
                                    base64::engine::general_purpose::STANDARD.encode(encoded);
                                send_mail(email.to_string(), otp.to_string());
                                HttpResponse::Ok().json(EmailTriggerResponse {
                                    verification_key: encoded_string.to_string(),
                                })
                            }
                            Err(_e) => HttpResponse::Ok().json(Error {
                                code: _e.to_string(),
                                desc: "Internal Server Error 000".to_string(),
                            }),
                        }
                    }
                    None => HttpResponse::Ok().json(Error {
                        code: "_e".to_string(),
                        desc: "Internal Server Error 111".to_string(),
                    }),
                }
            }
            Err(_e) => HttpResponse::Ok().json(Error {
                code: _e.to_string(),
                desc: "Internal Server Error 222".to_string(),
            }),
        }
    }
}

#[post("/verify/otp")]
async fn verify_otp(
    payload: web::Json<VerifyOtpRequest>,
    db: web::Data<SecuredAuthDatabase>,
) -> impl Responder {
    let current_time = DateTime::now();
    println!("{:#?}", payload);
    match (
        payload.verification_key.is_empty(),
        payload.otp.is_empty(),
        payload.check.is_empty(),
    ) {
        (false, false, false) => {
            let decoded_string = base64::engine::general_purpose::STANDARD
                .decode(payload.verification_key.to_string());
            // let decoded_string2 =  base64::engine::general_purpose::STANDARD
            //     .decode("t99C510LnefU9ibLf5ZWLTaRriPq8pP68wQNozFesdV9xg6K/zZ3qzj9zflDayV2Pl89JLsWrlOlJVOKxjk09tZbf9SKTq2XB6FD/RBm5chFJYSmAftcwoPVBAhC9WwgpYkgyWYZMZcfkDBc/Z3C361InuHg9MoWekCEDQI=");
            println!("after decoding  = {:?}", decoded_string);
            match decoded_string {
                Ok(encoded_object) => {
                    // println!("encoded_object object = {:?}", encoded_object);
                    let decoded_object = decrypt(encoded_object);
                    match decoded_object {
                        Ok(ob) => {
                            println!("oid = {:?}", ob.id.to_owned());
                            let oid = ObjectId::parse_str(ob.id);
                            match oid {
                                Ok(iad) => {
                                    let uni_id = bson::Bson::ObjectId(iad);
                                    println!("id = {}", uni_id.to_owned());
                                    let otp_instance = db.get_otp_by_id(uni_id.to_owned()).await;
                                    match otp_instance {
                                        Ok(option_otp) => match option_otp {
                                            Some(otp) => {
                                                if otp.verified {
                                                    HttpResponse::Ok().json(Error {
                                                        code: "OTP_ALREADY_VERIFIED".to_string(),
                                                        desc: "Internal Server Error".to_string(),
                                                    });
                                                }

                                                if otp.expiration_time >= current_time {
                                                    HttpResponse::Ok().json(Error {
                                                        code: "OTP_EXPIRED".to_string(),
                                                        desc: "Internal Server Error".to_string(),
                                                    });
                                                }

                                                if otp.otp == payload.otp {
                                                    HttpResponse::Ok().json(Error {
                                                        code: "WRONG_OTP_ENTERED".to_string(),
                                                        desc: "Internal Server Error".to_string(),
                                                    });
                                                }
                                                let updated_otp =
                                                    db.mark_otp_as_verified(uni_id.clone()).await;
                                                match updated_otp {
                                                    Ok(_up) => HttpResponse::Ok().json(
                                                        EmailVerifyResponse {
                                                            status: "SUCCESS".to_string(),
                                                            message: "OTP verification successfull"
                                                                .to_string(),
                                                        },
                                                    ),
                                                    Err(_e) => HttpResponse::Ok().json(Error {
                                                        code: _e.to_string(),
                                                        desc: "Internal Server Error".to_string(),
                                                    }),
                                                }
                                            }
                                            None => HttpResponse::Ok().json(Error {
                                                code: "e.to_string()".to_string(),
                                                desc: "Internal Server Error".to_string(),
                                            }),
                                        },
                                        Err(_e) => HttpResponse::Ok().json(Error {
                                            code: _e.to_string(),
                                            desc: "Internal Server Error - otp instance not found"
                                                .to_string(),
                                        }),
                                    }
                                }
                                Err(_e) => HttpResponse::Ok().json(Error {
                                    code: _e.to_string(),
                                    desc: "Internal Server Error - otp decode failed ".to_string(),
                                }),
                            }
                        }
                        Err(_e) => HttpResponse::Ok().json(Error {
                            code: _e.to_string(),
                            desc: "Internal Server Error - otp decode failed ".to_string(),
                        }),
                    }
                }
                Err(_e) => HttpResponse::Ok().json(Error {
                    code: _e.to_string(),
                    desc: "Internal Server Error - key failed ".to_string(),
                }),
            }
        }
        _ => HttpResponse::Ok().json(Error {
            code: "PAYLOAD_WRONG".to_string(),
            desc: "Internal Server Error".to_string(),
        }),
    }
}
