use std::convert::Infallible;

use chrono::Local;
use derive_more::{Display, TryInto};
use warp::{
    http::{HeaderValue, StatusCode},
    reply::Response,
};
// use log::info;
use serde::{Deserialize, Serialize};
use warp::{
    self,
    reject::{MethodNotAllowed, PayloadTooLarge, UnsupportedMediaType},
    reply::{self},
    Rejection, Reply,
};

#[derive(Clone, Debug, Display, TryInto)]
pub enum ApiError {
    InternalError,
    #[display(fmt = "Db")]
    DbConnection,
    #[display(fmt = "Unauthorized")]
    Unauthorized(String),
}

impl warp::reject::Reject for ApiError {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ErrorResponse {
    fn new(code: StatusCode, message: String) -> Self {
        Self {
            code: code.as_u16(),
            message,
        }
    }
}

impl ApiError {
    fn status(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Reply for ApiError {
    fn into_response(self) -> Response {
        let json = warp::reply::json(&ErrorResponse::from(&self));

        warp::reply::with_status(json, self.status()).into_response()
    }
}

impl From<&ApiError> for ErrorResponse {
    fn from(error: &ApiError) -> Self {
        println!("{:?}", error);
        ErrorResponse {
            code: error.status().as_u16(),
            message: match error {
                ApiError::InternalError => String::from(""),
                ApiError::DbConnection => String::from("Db"),
                _ => String::try_from(error.clone()).unwrap_or("Not Reco".to_owned()),
            },
        }
    }
}

impl From<&str> for ErrorResponse {
    fn from(err: &str) -> Self {
        let _date = Local::now();
        ErrorResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: String::from(err),
        }
    }
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let body;
    // info!("rejection handler: {:?}", err);
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        body = reply::json(&ErrorResponse::new(code, "Not Found".to_owned()));
    } else if let Some(e) = err.find::<ApiError>() {
        code = e.status();
        body = reply::json(&ErrorResponse::from(e));
    } else if let Some(cause) = err.find::<warp::cors::CorsForbidden>() {
        code = StatusCode::FORBIDDEN;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else if let Some(cause) = err.find::<warp::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else if let Some(cause) = err.find::<warp::reject::LengthRequired>() {
        code = StatusCode::LENGTH_REQUIRED;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else if let Some(cause) = err.find::<PayloadTooLarge>() {
        code = StatusCode::PAYLOAD_TOO_LARGE;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else if let Some(cause) = err.find::<UnsupportedMediaType>() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else if let Some(cause) = err.find::<MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        body = reply::json(&ErrorResponse::new(code, cause.to_string()));
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        body = reply::json(&ErrorResponse::new(
            code,
            format!("unexpected error: {:?}", err),
        ));
    }
    let mut rep = reply::with_status(body, code).into_response();
    rep.headers_mut()
        .insert("access-control-allow-origin", HeaderValue::from_static("*"));
    Ok(rep)
}
