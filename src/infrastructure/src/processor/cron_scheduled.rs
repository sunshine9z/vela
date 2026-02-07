use commonx::error::AppError;
use sea_orm::JsonValue;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, OnceLock};
use tokio_cron_scheduler::JobScheduler;

pub static GLOBAL_SCHEDULER: OnceLock<Arc<CronScheduled>> = OnceLock::new();
// #[derive(Serialize,Deserialize,Debug,Clone)]
// pub struct PeriodicJob{
//     pub name: String,
//     pub class: String,
//     pub cron: String,
//     pub queue: Option<String>,
//     pub args: Option<String>,
//     retry: Option<bool>,
//
//
//     #[serde(skip)]
//     cron_schedule: Option<Cron>,
// }
//
//
// impl PeriodicJob {
//     pub fn from_json_string(job: &String) -> Result<PeriodicJob, AppError> {
//         let mut job:Self = serde_json::from_str(job.as_str())?;
//         job.hydrate_attributes()?;
//         Ok(job)
//     }
//
//     /// 补充运行时属性
//     fn hydrate_attributes(&mut self) -> Result<(),AppError> {
//         self.hydrate_cron_schedule()?;
//         self.hydrate_json_args()?;
//         Ok(())
//     }
//
//     /// 补充 cron 调度器
//     fn hydrate_cron_schedule(&mut self) -> Result<(), AppError> {
//         self.cron_schedule = Some(Cron::from_str(&self.cron)?);
//         Ok(())
//     }
//
// }

pub struct CronScheduled {
    scheduler: Mutex<JobScheduler>,
    hasStart: bool,
}

impl CronScheduled {
    pub fn new(scheduler: JobScheduler) -> Self {
        Self {
            scheduler: Mutex::new(scheduler),
            hasStart: false,
        }
    }

    pub async fn start(&mut self) -> Result<(), AppError> {
        if self.hasStart {
            return Ok(());
        }
        let mut s = self.scheduler.lock().unwrap();
        if self.hasStart {
            return Ok(());
        }

        self.hasStart = true;

        s.start().await.unwrap();
        Ok(())
    }
}

pub async fn init_corn_schedule() -> Result<(), AppError> {
    let sched = JobScheduler::new().await.unwrap();
    GLOBAL_SCHEDULER.get_or_init(|| Arc::new(CronScheduled::new(sched)));
    Ok(())
}
