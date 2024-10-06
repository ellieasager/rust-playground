use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub db_client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum KickstarterError {
    InternalServer,
    ItemNotFound,
    BadRequest(String),
}

impl fmt::Display for KickstarterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            KickstarterError::InternalServer => "internal service error",
            KickstarterError::ItemNotFound => "item not found",
            KickstarterError::BadRequest(msg) => msg,
        };
        write!(f, "{:?}", message)
    }
}

impl ResponseError for KickstarterError {
    fn error_response(&self) -> HttpResponse {
        match self {
            KickstarterError::InternalServer => HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(self.to_string()),
            KickstarterError::ItemNotFound => HttpResponse::NotFound()
                .content_type("text/plain")
                .body(self.to_string()),
            KickstarterError::BadRequest(_) => HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(self.to_string()),
        }
    }
}
