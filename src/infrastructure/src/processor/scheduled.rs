use commonx::error::AppError;
use crate::cache::CacheManager;
use crate::processor::unit_of_work::UnitOfWork;

#[derive(Default)]
pub struct SortedScheduledWork {}

impl SortedScheduledWork {
    pub async fn enqueue_jobs(
        &self,
        now: chrono::DateTime<chrono::Local>,
        sorted_set: &Vec<String>,
    ) -> Result<usize,AppError> {
        let mut n = 0;
        let cache = CacheManager::instance();
        for s in sorted_set {
            let jobs: Vec<String> = cache.zrangebyscore_limit(
                s,
                f64::NEG_INFINITY,
                now.timestamp() as f64,
                0, 100
            ).await?;
            n += jobs.len();

            for job in jobs {
                if cache.zrem(s, job.clone()).await? {
                    let work = UnitOfWork::from_job_string(job)?;
                    work.enqueue_direct().await?;
                }
            }

        }

        Ok(n)
    }
}
