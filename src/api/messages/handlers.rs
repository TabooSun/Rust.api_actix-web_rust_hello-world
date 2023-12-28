use std::collections::HashSet;

use actix_web::{get, Responder, web};

use actix_web_auth0::extractors::Claims;
use contracts::dto::error_response_dto::ErrorResponseDto;
use contracts::dto::message_dto::MessageDto;
use contracts::error_code::error_code::ErrorCode;

#[utoipa::path()]
#[get("/admin")]
pub async fn admin(claims: Claims) -> impl Responder {
    if claims.validate_permissions(&HashSet::from(["read:admin-messages".to_string()])) {
        Ok(web::Json(MessageDto {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-role-based-access-control".to_string(),
            text: "The secured API requires a valid access token and the read:admin-messages permission to share this admin message.".to_string(),
        }))
    } else {
        Err(ErrorResponseDto {
            error_code: ErrorCode::InsufficientPermissions,
            error_description: Some("Requires read:admin-messages".to_string()),
            message: "Permission denied".to_string(),
            status_code: actix_web::http::StatusCode::FORBIDDEN,
        })
    }
}

#[utoipa::path()]
#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token to share this protected message.".to_string(),
    })
}

#[utoipa::path()]
#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API doesn't require an access token to share this public message.".to_string(),
    })
}
