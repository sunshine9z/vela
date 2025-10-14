use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use commonx::error::AppError;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VQuery<T>(pub T);
impl<T, S> FromRequestParts<S> for VQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, _state).await?;
        value.validate()?;
        Ok(VQuery(value))
    }
}
