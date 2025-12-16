use axum::extract::{Form,FromRequest,Request};
use serde::de::DeserializeOwned;
use validator::Validate;
use commonx::error::AppError;
use crate::resp::ApiResponse;

#[derive(Debug,Clone,Copy,Default)]
pub struct VForm<T>(pub T);

impl<T, S> FromRequest<S> for VForm<T>
where T: DeserializeOwned + Validate,
    S: Send+Sync,{
    type Rejection = ApiResponse<()>;
    async fn from_request(req: Request<>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        value
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(VForm(value))
    }
}