use crate::processor::job::Job;
use commonx::error::AppError;

#[derive(Debug)]
pub struct UnitOfWork {
    pub queue: String,
    pub job: Job,
}

impl UnitOfWork {
    pub fn from_job_string(job_str: String) -> Result<Self, AppError> {
        let job: Job = serde_json::from_str(job_str.as_str())?;
        Ok(job.into())
    }
}

impl From<Job> for UnitOfWork {
    fn from(job: Job) -> Self {
        UnitOfWork {
            queue: format!("queue:{}", &job.queue),
            job,
        }
    }
}
