pub mod cron_scheduled;

use std::sync::{Arc, OnceLock};

use commonx::error::AppError;
use tokio_cron_scheduler::JobScheduler;

use crate::cron_scheduled::cron_scheduled::CronScheduled;

pub static GLOBAL_SCHEDULER: OnceLock<Arc<CronScheduled>> = OnceLock::new();

pub async fn instance() -> Result<(), AppError> {
    let sched = JobScheduler::new().await.unwrap();
    GLOBAL_SCHEDULER.get_or_init(|| Arc::new(CronScheduled::new(sched)));
    Ok(())
}
