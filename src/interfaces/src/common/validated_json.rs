use axum::{
    Json,
    extract::{FromRequest, Request},
};
use commonx::error::AppError;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::resp::ApiResponse;

#[derive(Debug, Clone, Copy, Default)]
pub struct VJson<T>(pub T);

impl<T, S> FromRequest<S> for VJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(VJson(value))
    }
}
