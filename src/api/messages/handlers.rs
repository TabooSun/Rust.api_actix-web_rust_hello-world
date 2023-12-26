use std::collections::HashSet;

use actix_web::{get, Responder, web};
use actix_web::error::ErrorForbidden;

use crate::{extractors::Claims, types::ErrorMessage};

use super::types::Message;

#[utoipa::path()]
#[get("/admin")]
pub async fn admin(claims: Claims) -> impl Responder {
    if claims.validate_permissions(&HashSet::from(["read:admin-messages".to_string()])) {
        Ok(web::Json(Message {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "basic-role-based-access-control".to_string(),
            text: "The secured API requires a valid access token and the read:admin-messages permission to share this admin message.".to_string(),
        }))
    } else {
        Err(ErrorForbidden(ErrorMessage {
            error: Some("insufficient_permissions".to_string()),
            error_description: Some("Requires read:admin-messages".to_string()),
            message: "Permission denied".to_string(),
        }))
    }
}

#[utoipa::path()]
#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API requires a valid access token to share this protected message.".to_string(),
    })
}

#[utoipa::path()]
#[get("/public")]
pub async fn public() -> impl Responder {
        web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-role-based-access-control".to_string(),
        text: "The secured API doesn't require an access token to share this public message.".to_string(),
    })
}
