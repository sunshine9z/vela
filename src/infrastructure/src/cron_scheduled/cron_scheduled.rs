use commonx::error::AppError;
use std::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

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

use std::collections::HashMap;

pub struct CronScheduled {
    scheduler: Mutex<JobScheduler>,
    has_start: bool,
}

#[derive(Debug, Clone)]
pub struct JobStatus {
    pub job_id: String,
    pub is_running: bool,
}

impl CronScheduled {
    pub(crate) fn new(scheduler: JobScheduler) -> Self {
        Self {
            scheduler: Mutex::new(scheduler),
            has_start: false,
            // jobs: Mutex::new(HashMap::new()),
        }
    }

    pub async fn start(&mut self) -> Result<(), AppError> {
        if self.has_start {
            return Ok(());
        }
        let s = self.scheduler.lock().unwrap();
        if self.has_start {
            return Ok(());
        }

        self.has_start = true;

        s.start().await.unwrap();
        Ok(())
    }

    // 新增定时任务
    // pub async fn add_job(
    //     &self,
    //     job_id: String,
    //     cron_expression: &str,
    //     job_func: impl Fn() + Send + Sync + 'static,
    // ) -> Result<(), AppError> {
    //     let mut scheduler = self.scheduler.lock().unwrap();
    //     let mut jobs = self.jobs.lock().unwrap();

    //     // 创建定时任务
    //     let job = scheduler
    //         .new_cron_job(cron_expression)
    //         .unwrap()
    //         .do_job(move |_uuid, _l| {
    //             job_func();
    //         })
    //         .await
    //         .unwrap();

    //     // 保存任务句柄
    //     jobs.insert(job_id, job);
    //     Ok(())
    // }

    // 删除定时任务
    // pub async fn remove_job(&self, job_id: &str) -> Result<bool, AppError> {
    //     let mut jobs = self.jobs.lock().unwrap();
    //     if let Some(job_handle) = jobs.remove(job_id) {
    //         job_handle.abort().await.unwrap();
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }

    // 罗列所有定时任务及其状态
    // pub fn list_jobs(&self) -> Result<Vec<JobStatus>, AppError> {
    //     let jobs = self.jobs.lock().unwrap();
    //     let mut status_list = Vec::new();

    //     for (job_id, _job_handle) in jobs.iter() {
    //         // 注意：tokio-cron-scheduler 的 JobHandle 没有直接提供运行状态查询
    //         // 这里我们假设任务在添加后都是活跃的
    //         status_list.push(JobStatus {
    //             job_id: job_id.clone(),
    //             is_running: true,
    //         });
    //     }

    //     Ok(status_list)
    // }
}

/// 初始化定时任务调度器
pub async fn init_corn_schedule() -> Result<(), AppError> {
    let scheduler = JobScheduler::new().await.unwrap();
    let mut cron_scheduled = CronScheduled::new(scheduler);
    cron_scheduled.start().await?;
    Ok(())
}
