use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod auth_jwt;
pub mod corn_job;
pub mod user_info;

#[derive(Debug, Serialize, Deserialize, Clone, Validate, Default)]
pub struct GetByIdReq {
    pub id: i64,
}
