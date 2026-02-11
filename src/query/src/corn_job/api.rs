use async_trait::async_trait;
use commonx::error::AppError;

use crate::corn_job::entity::{JobVo, ListJobQo};

#[async_trait]
pub trait JobQueryTrait {
    async fn get_by_id(&self, id: i64) -> Result<Option<JobVo>, AppError>;
    async fn list(&self, query: ListJobQo) -> Result<Vec<JobVo>, AppError>;
}
