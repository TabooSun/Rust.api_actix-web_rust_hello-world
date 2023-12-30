use serde::Serialize;
use strum_macros::{Display, EnumString};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumString, Display, ToSchema, ToResponse)]
pub enum ErrorCode {
    OidcError,
    InsufficientPermissions,
    InvalidToken,
    InternalServerError,
    NotFound,
}
