use async_trait::async_trait;

use chrono::{Local, TimeZone};
use commonx::error::AppError;
use job_domain::commons::error::JobDomainError;
use job_domain::entity::job::{CreateJobDto, UpdateJobDto};
use job_domain::repository::job::JobRepositoryTrait;
use job_domain::{JobDomainImpl, new_job_domain};
use queryx::corn_job::api::JobQueryTrait;
use queryx::corn_job::entity::{JobVo, ListJobQo};
use queryx::corn_job::services::JobQueryImpl;

use crate::persistence::entities::corn_job::Model as CornJobModel;

pub struct JobDomainRepositoryImpl {}

impl From<CornJobModel> for CreateJobDto {
    fn from(model: CornJobModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            class: model.class,
            cron: model.cron,
            queue: model.queue,
            args: model.args,
            retry: model.retry,
            created_at: Local
                .from_local_datetime(&model.created_at)
                .single()
                .unwrap_or_default(),
            updated_at: Local
                .from_local_datetime(&model.updated_at)
                .single()
                .unwrap_or_default(),
        }
    }
}

impl From<CornJobModel> for JobVo {
    fn from(model: CornJobModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            class: model.class,
            cron: model.cron,
            queue: model.queue,
            args: model.args,
            retry: model.retry,
            created_at: Local
                .from_local_datetime(&model.created_at)
                .single()
                .unwrap_or_default(),
            updated_at: Local
                .from_local_datetime(&model.updated_at)
                .single()
                .unwrap_or_default(),
        }
    }
}

#[async_trait]
impl JobQueryTrait for JobDomainRepositoryImpl {
    async fn list(&self, query: ListJobQo) -> Result<Vec<JobVo>, AppError> {
        CornJobModel::list(query.page_req.page, query.page_req.page_size)
            .await
            .map_err(|e| e.into())
            .map(|models| models.into_iter().map(JobVo::from).collect())
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<JobVo>, AppError> {
        CornJobModel::find_by_id(id)
            .await
            .map_err(|e| e.into())
            .map(|model| model.map(JobVo::from))
    }
}

#[async_trait]
impl JobRepositoryTrait for JobDomainRepositoryImpl {
    async fn create(&self, job: CreateJobDto) -> Result<i64, JobDomainError> {
        CornJobModel::create(
            job.name, job.class, job.cron, job.queue, job.args, job.retry,
        )
        .await
        .map_err(|e| JobDomainError::DbError(e.to_string()))
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), JobDomainError> {
        CornJobModel::delete_by_id(id)
            .await
            .map_err(|e| JobDomainError::DbError(e.to_string()))
    }

    async fn update_by_id(&self, id: i64, job: UpdateJobDto) -> Result<(), JobDomainError> {
        CornJobModel::update_by_id(
            id, job.name, job.class, job.cron, job.queue, job.args, job.retry,
        )
        .await
        .map_err(|e| JobDomainError::DbError(e.to_string()))
    }
}

pub fn new_job_domain_service() -> JobDomainImpl {
    new_job_domain(Box::new(JobDomainRepositoryImpl {}))
}

pub fn new_job_query_service() -> JobQueryImpl {
    JobQueryImpl::new(Box::new(JobDomainRepositoryImpl {}))
}
