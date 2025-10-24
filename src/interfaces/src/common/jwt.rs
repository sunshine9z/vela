use chrono::Local;
use commonx::error::AppError;
use infrastructurex::config::APP_CONFIG;
use jsonwebtoken::{Header, encode};
use serde::{Deserialize, Serialize};

use crate::types::{auth_jwt::Claims, user_info::KEYS};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    pub token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64) -> Self {
        Self {
            token: access_token,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}

pub async fn authorize(mut payload: Claims) -> Result<AuthBody, AppError> {
    let iat = Local::now();

    let exp = iat.timestamp() + APP_CONFIG.auth.jwt.expiration;
    payload.exp = exp;
    // Create the authorization token
    let token = encode(&Header::default(), &payload, &KEYS.encoding)
        .map_err(|_| AppError::AuthError("授权错误".to_string()))?;
    // Send the authorized token
    Ok(AuthBody::new(
        token,
        payload.exp,
        APP_CONFIG.auth.jwt.expiration,
    ))
}
