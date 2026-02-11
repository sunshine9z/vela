use crate::corn_job::{
    MODEL_JOB_QUERY,
    api::JobQueryTrait,
    entity::{JobVo, ListJobQo},
};
use async_trait::async_trait;
use commonx::error::AppError;
use tracing::info;

pub struct JobQueryImpl {
    pub job_repo: Box<dyn JobQueryTrait + Sync + Send>,
}

#[async_trait]
impl JobQueryTrait for JobQueryImpl {
    async fn list(&self, query: ListJobQo) -> Result<Vec<JobVo>, AppError> {
        info!(target: MODEL_JOB_QUERY, "Listing all jobs");
        self.job_repo.list(query).await
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<JobVo>, AppError> {
        info!(target: MODEL_JOB_QUERY, "Finding job with id: {}", id);
        self.job_repo.get_by_id(id).await
    }
}

impl JobQueryImpl {
    pub fn new(job_repo: Box<dyn JobQueryTrait + Sync + Send>) -> Self {
        Self { job_repo }
    }
}
