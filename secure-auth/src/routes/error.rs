use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize)]
pub enum MyError {
    // #[display(fmt = "internal error")]
    // InternalError,

    // #[display(fmt = "bad request")]
    // BadClientData,

    // #[display(fmt = "timeout")]
    // Timeout,
    // #[display(fmt = "validation error")]
    // ValidationError,
    #[display(fmt = "efw")]
    Error { code: String, desc: String },
}

// impl error::ResponseError for MyError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match *self {
//             MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MyError::BadClientData => StatusCode::BAD_REQUEST,
//             MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         }
//     }
// }
