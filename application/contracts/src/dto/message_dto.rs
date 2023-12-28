use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageDto {
    pub api: String,
    pub branch: String,
    pub text: String,
}
