use std::fmt::{Display, Formatter};

use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::Serialize;

use crate::error_code::error_code::ErrorCode;

#[derive(Serialize, Debug, thiserror::Error)]
pub struct ErrorResponseDto {
    pub error_code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    // TODO: Localize this message.
    pub message: String,

    #[serde(skip)]
    pub status_code: StatusCode,
}

impl Responder for ErrorResponseDto {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

impl ResponseError for ErrorResponseDto {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).content_type(ContentType::json()).json(self)
    }
}

impl Display for ErrorResponseDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

