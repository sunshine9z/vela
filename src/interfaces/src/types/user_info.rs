use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::TypedHeader;
use commonx::error::AppError;
use configx::APP_CONFIG;
use headers::{Authorization, authorization::Bearer};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode, errors::ErrorKind};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &APP_CONFIG.auth.jwt.secret;
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub id: i64,
    pub role: String,
    pub token: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub id: i64,
    pub token: String,
    pub role: String,
    pub exp: i64,
}

impl<S> FromRequestParts<S> for UserInfo
where
    S: Send + Sync,
{
    type Rejection = AppError;
    /// 将用户信息注入request
    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let token_v = get_bear_token(parts).await?;
            let token_data =
                match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
                    Ok(token) => token,
                    Err(err) => match err.kind() {
                        ErrorKind::InvalidToken => {
                            return Err(AppError::AuthError("token错误,请重新登录".to_string()));
                        }
                        ErrorKind::ExpiredSignature => {
                            return Err(AppError::AuthError("token过期,请重新登录".to_string()));
                        }
                        _ => {
                            tracing::info!("AuthError:{:?}", err);
                            return Err(AppError::AuthError("token错误,请重新登录".to_string()));
                        }
                    },
                };
            let claims: Claims = token_data.claims;
            tracing::info!(" userinfo.id:{:?}", claims.id);
            let user = UserInfo {
                username: claims.username,
                id: claims.id,
                role: claims.role,
                token: claims.token,
            };
            parts.extensions.insert(user.clone());
            Ok(user)
        }
    }
}

pub async fn get_bear_token(parts: &mut Parts) -> Result<String, AppError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AppError::AuthError("token错误,请重新登录".to_string()))?;
    // Decode the user data
    let bearer_data: &str = bearer.token();
    Ok(bearer_data.to_owned())
}
