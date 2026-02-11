use async_trait::async_trait;

use crate::MODEL_JOB_DOMAIN;
use crate::entity::job::UpdateJobDto;
use crate::{
    api::traits::JobDomainTrait, commons::error::JobDomainError, entity::job::CreateJobDto,
};
use tracing::info;

pub struct JobDomainImpl {
    pub job_repo: Box<dyn crate::repository::job::JobRepositoryTrait + Sync + Send>,
}

#[async_trait]
impl JobDomainTrait for JobDomainImpl {
    async fn create(&self, job: CreateJobDto) -> Result<i64, JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Creating job: {}", job.name);
        self.job_repo.create(job).await
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Deleting job with id: {}", id);
        self.job_repo.delete_by_id(id).await
    }

    async fn update_by_id(&self, id: i64, job: UpdateJobDto) -> Result<(), JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Updating job with id: {}", id);
        self.job_repo.update_by_id(id, job).await
    }
}
