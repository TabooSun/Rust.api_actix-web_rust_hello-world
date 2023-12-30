use serde::Serialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, EnumString, Display)]
pub enum MessagePermissions {
    #[strum(serialize = "read:admin_messages")]
    ReadAdminMessages,
}
