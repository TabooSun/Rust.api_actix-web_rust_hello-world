use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct MessageDto {
    pub api: String,
    pub branch: String,
    /// The freaking text.
    pub text: String,
}

impl Responder for MessageDto {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
