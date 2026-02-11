//! Job Domain Module

pub mod api;
pub mod commons;
pub mod entity;
pub mod repository;
pub mod services;

pub use api::traits::JobDomainTrait;
pub use services::service::JobDomainImpl;

pub const MODEL_JOB_DOMAIN: &str = "JOB_DOMAIN";

pub fn new_job_domain(
    job_repo: Box<dyn repository::job::JobRepositoryTrait + Sync + Send>,
) -> JobDomainImpl {
    JobDomainImpl { job_repo }
}
