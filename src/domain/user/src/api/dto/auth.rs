use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDto {
    pub username: String,
    pub password: String,
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDtoWithCaptcha {
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub captcha: String,
}
