use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use commonx::error::AppError;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::resp::ApiResponse;

#[derive(Debug, Clone, Copy, Default)]
pub struct VQuery<T>(pub T);
impl<T, S> FromRequestParts<S> for VQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, _state)
            .await
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(VQuery(value))
    }
}
