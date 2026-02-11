use commonx::error::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobDomainError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Job not found")]
    JobNotFound,
    #[error("Invalid job data: {0}")]
    InvalidJobData(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<JobDomainError> for AppError {
    fn from(err: JobDomainError) -> Self {
        Self::InternalError(err.to_string())
    }
}
