use std::collections::HashSet;

use actix_web::{get, Responder};

use actix_web_auth0::extractors::Claims;
use contracts::dto::error_response_dto::ErrorResponseDto;
use contracts::dto::message_dto::MessageDto;

#[utoipa::path()]
#[get("/admin")]
#[auto_log::auto_log]
pub async fn admin(claims: Claims) -> Result<MessageDto, ErrorResponseDto> {
    claims.authorize(&HashSet::from(["read:admin-messages".to_string()]))?;

    Ok(MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token and the read:admin-messages permission to share this admin message.".to_string(),
    })
}

#[utoipa::path()]
#[get("/protected")]
#[auto_log::auto_log]
pub async fn protected(_claims: Claims) -> impl Responder {
    MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token to share this protected message.".to_string(),
    }
}

#[utoipa::path()]
#[get("/public")]
#[auto_log::auto_log]
pub async fn public() -> impl Responder {
    MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API doesn't require an access token to share this public message.".to_string(),
    }
}
