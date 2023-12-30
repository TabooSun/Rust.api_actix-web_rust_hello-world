use std::collections::HashSet;

use actix_web::{get, Responder};

use actix_web_auth0::extractors::Claims;
use contracts::dto::error_response_dto::ErrorResponseDto;
use contracts::dto::message_dto::MessageDto;
use contracts::permissions::message_permissions::MessagePermissions;

/// Admin API. This API can be accessed with an access token and the read:admin-messages permission.
#[utoipa::path(
path = "/api/messages/admin",
responses(
(status = 200, description = "Message retrieved successfully", body = MessageDto),
(status = 401, description = "Bad credentials. The credential given is invalid.", body = ErrorResponseDto),
(status = 403, description = "Forbidden. The credential given is valid but doesn't have the required permission.", body = ErrorResponseDto),
),
security(
("OAuth2" = ["read:admin_messages"])
)
)]
#[get("/admin")]
#[auto_log::auto_log]
pub async fn admin(claims: Claims) -> Result<MessageDto, ErrorResponseDto> {
    claims.authorize(&HashSet::from([MessagePermissions::ReadAdminMessages.to_string()]))?;

    Ok(MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token and the read:admin-messages permission to share this admin message.".to_string(),
    })
}

/// Protected API. This API can be accessed with an access token.
#[utoipa::path(
path = "/api/messages/protected",
responses(
(status = 200, description = "Message retrieved successfully", body = MessageDto),
(status = 401, description = "Bad credentials. The credential given is invalid.", body = ErrorResponseDto),
),
security(
("OAuth2" = ["read:admin_messages"])
)
)]
#[get("/protected")]
#[auto_log::auto_log]
pub async fn protected(_claims: Claims) -> impl Responder {
    MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token to share this protected message.".to_string(),
    }
}

/// Create public API. This API can be accessed without an access token.
#[utoipa::path(
path = "/api/messages/public",
responses(
(status = 200, description = "Message retrieved successfully", body = MessageDto),
),
)]
#[get("/public")]
#[auto_log::auto_log]
pub async fn public() -> impl Responder {
    MessageDto {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API doesn't require an access token to share this public message.".to_string(),
    }
}
