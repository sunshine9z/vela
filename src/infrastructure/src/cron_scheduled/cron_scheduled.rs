use commonx::error::AppError;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

/// 定时任务状态
#[derive(Debug, Clone)]
pub struct JobStatus {
    /// 任务ID
    pub job_id: String,
    /// 是否正在运行
    pub is_running: bool,
}

/// 定时任务调度器
///
/// 负责管理定时任务的调度和执行
pub struct CronScheduled {
    /// 任务调度器
    scheduler: Mutex<JobScheduler>,
    /// 任务句柄映射
    jobs: Mutex<HashMap<String, ()>>,
    /// 是否已启动
    has_start: bool,
}

impl CronScheduled {
    /// 创建定时任务调度器实例
    ///
    /// # 参数
    /// - `scheduler`: 任务调度器
    ///
    /// # 返回
    /// 定时任务调度器实例
    pub(crate) fn new(scheduler: JobScheduler) -> Self {
        Self {
            scheduler: Mutex::new(scheduler),
            jobs: Mutex::new(HashMap::new()),
            has_start: false,
        }
    }

    /// 启动定时任务调度器
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回应用错误
    pub async fn start(&mut self) -> Result<(), AppError> {
        if self.has_start {
            return Ok(());
        }

        let scheduler = self.scheduler.lock().await;
        if self.has_start {
            return Ok(());
        }

        scheduler.start().await.map_err(|e| AppError::from(e))?;
        self.has_start = true;
        Ok(())
    }

    /// 新增定时任务
    ///
    /// # 参数
    /// - `job_id`: 任务ID
    /// - `cron_expression`: Cron表达式
    /// - `job_func`: 任务执行函数
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回应用错误
    pub async fn add_job(
        &self,
        job_id: String,
        cron_expression: &str,
        job_func: impl Fn() + Send + Sync + 'static,
    ) -> Result<(), AppError> {
        // let mut scheduler = self.scheduler.lock().await;
        // let mut jobs = self.jobs.lock().await;

        // // 创建定时任务
        // let job = scheduler
        //     .new_cron_job(cron_expression)
        //     .map_err(|e| AppError::from(e))?
        //     .do_job(move |_uuid, _l| {
        //         job_func();
        //     })
        //     .await
        //     .map_err(|e| AppError::from(e))?;

        // // 保存任务句柄
        // jobs.insert(job_id, job);
        // Ok(())
        todo!()
    }

    /// 删除定时任务
    ///
    /// # 参数
    /// - `job_id`: 任务ID
    ///
    /// # 返回
    /// - 成功：返回是否删除成功
    /// - 失败：返回应用错误
    pub async fn remove_job(&self, job_id: &str) -> Result<bool, AppError> {
        // let mut jobs = self.jobs.lock().await;
        // if let Some(job_handle) = jobs.remove(job_id) {
        //     job_handle.abort().await.map_err(|e| AppError::from(e))?;
        //     Ok(true)
        // } else {
        //     Ok(false)
        // }

        todo!()
    }

    /// 罗列所有定时任务及其状态
    ///
    /// # 返回
    /// - 成功：返回任务状态列表
    /// - 失败：返回应用错误
    pub async fn list_jobs(&self) -> Result<Vec<JobStatus>, AppError> {
        let jobs = self.jobs.lock().await;
        let mut status_list = Vec::with_capacity(jobs.len());

        for (job_id, _job_handle) in jobs.iter() {
            // 注意：tokio-cron-scheduler 的 JobHandle 没有直接提供运行状态查询
            // 这里我们假设任务在添加后都是活跃的
            status_list.push(JobStatus {
                job_id: job_id.clone(),
                is_running: true,
            });
        }

        Ok(status_list)
    }

    /// 检查任务是否存在
    ///
    /// # 参数
    /// - `job_id`: 任务ID
    ///
    /// # 返回
    /// 任务是否存在
    pub async fn exists(&self, job_id: &str) -> bool {
        let jobs = self.jobs.lock().await;
        jobs.contains_key(job_id)
    }
}

/// 初始化定时任务调度器
///
/// # 返回
/// - 成功：返回空
/// - 失败：返回应用错误
pub async fn init_corn_schedule() -> Result<(), AppError> {
    let scheduler = JobScheduler::new().await.map_err(|e| AppError::from(e))?;
    let mut cron_scheduled = CronScheduled::new(scheduler);
    cron_scheduled.start().await
}
