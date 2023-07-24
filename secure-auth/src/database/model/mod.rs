use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptSharedKey};
use validator::Validate;

fn current_date() -> DateTime {
    DateTime::now()
}

fn expiration_time() -> DateTime {
    // let duration = Duration::from_millis(172800000);
    DateTime::from_millis(DateTime::now().timestamp_millis() + 300000)
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct OTP {
    pub otp: String,
    pub verified: bool,

    #[validate(email)]
    pub email: String,
    #[serde(default = "current_date")]
    pub created_at: DateTime,
    #[serde(default = "expiration_time")]
    pub expiration_time: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedObject {
    pub message: String,
    pub id: String,
    pub check: String,
}

impl SerdeEncryptSharedKey for EncodedObject {
    type S = BincodeSerializer<Self>; // you can specify serializer implementation (or implement it by yourself).
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTriggerResponse {
    pub verification_key: String,
}
