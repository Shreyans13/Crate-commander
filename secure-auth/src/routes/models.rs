// use crate::routes::error::UserError::InternalError;
// use serde::de::{self};
use serde::Deserialize;
// use std;
// use std::fmt::{self};

// use super::error::UserError;

#[derive(Debug, Deserialize)]
pub struct Email {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOtpRequest {
    pub verification_key: String,
    pub otp: String,
    pub check: String,
}
