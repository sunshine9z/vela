use async_trait::async_trait;

use crate::commons::error::JobDomainError;
use crate::entity::job::{CreateJobDto, UpdateJobDto};

#[async_trait]
pub trait JobRepositoryTrait {
    async fn create(&self, job: CreateJobDto) -> Result<i64, JobDomainError>;
    async fn delete_by_id(&self, id: i64) -> Result<(), JobDomainError>;
    async fn update_by_id(&self, id: i64, job: UpdateJobDto) -> Result<(), JobDomainError>;
}
