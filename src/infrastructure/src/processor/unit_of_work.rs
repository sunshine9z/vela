use crate::{cache::CacheManager, processor::job::Job};
use commonx::error::AppError;
use sha2::{Digest, Sha256};

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
    pub async fn enqueue(&self) -> Result<(), AppError> {
        self.enqueue_direct().await
    }

    // 加入任务队列
    pub async fn enqueue_direct(&self) -> Result<(), AppError> {
        let mut job = self.job.clone();
        job.enqueued_at = Some(chrono::Local::now().timestamp() as f64);
        let cache = CacheManager::instance();
        if let Some(duration) = job.unique_for {
            let args_as_json_string: String = serde_json::to_string(&job.args)?;
            let args_hash = format!("{:x}", Sha256::digest(&args_as_json_string));
            let redis_key = format!(
                "enqueue:unique:{}:{}:{}",
                &job.queue, &job.class, &args_hash
            );
            if cache
                .set_nx_ex(&redis_key, "", duration.as_secs() as usize)
                .await?
            {
                return Ok(());
            }
        }
        cache.sadd("queue", &[job.queue.as_str()]).await?;
        cache
            .lpush(&self.queue, serde_json::to_string(&job)?)
            .await?;
        Ok(())
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
