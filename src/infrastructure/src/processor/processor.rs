use crate::cache::CacheManager;
use crate::processor::job::Job;
use crate::processor::periodic::PeriodicJob;
use crate::processor::unit_of_work::UnitOfWork;
use crate::processor::worker::{Worker, WorkerRef};
use crate::{web_error, web_info};
use commonx::error::AppError;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WorkFetcher {
    NoWorkFound,
    Done,
}

#[derive(Clone)]
pub struct Processor {
    queues: Vec<String>,
    periodic_jobs: Vec<PeriodicJob>,
    workers: BTreeMap<String, Arc<WorkerRef>>,
    cancellation_token: CancellationToken,
    num_workers: u16,
}

impl Processor {
    pub fn new(queues: Vec<String>, num_workers: u16) -> Self {
        Processor {
            queues: queues
                .iter()
                .map(|queue| format!("queue:{queue}"))
                .collect(),
            periodic_jobs: vec![],
            workers: BTreeMap::new(),
            cancellation_token: CancellationToken::new(),
            num_workers,
        }
    }

    async fn fetch(&self) -> Result<Option<UnitOfWork>, AppError> {
        let cache = CacheManager::instance();
        let res: Option<(String, String)> = cache.brpop(&self.queues, 2).await?;
        if let Some((queue, job_raw)) = res {
            let job: Job = serde_json::from_str(&job_raw)?;
            return Ok(Some(UnitOfWork { queue, job }));
        }
        Ok(None)
    }

    pub fn register<W: Worker + 'static>(&mut self, worker: W) {
        web_info!(" -- 注册进程: worker:{}", W::class_name());
        self.workers
            .insert(W::class_name(), Arc::new(WorkerRef::new(worker)));
    }

    pub async fn run(self) {
        let mut join_set = tokio::task::JoinSet::new();

        for i in 0..self.num_workers {
            join_set.spawn({
                let processor = self.clone();
                let cancellation_token = self.cancellation_token.clone();
                async move {
                    while !cancellation_token.is_cancelled() {
                        if let Err(err) = &processor.process_one().await {
                            web_error!(" -- 进程 {} 处理失败: {:?}", i, err);
                        }
                        web_info!(" -- 进程 {} cancelled...", i);
                    }
                }
            });
        }

        while let Some(res) = join_set.join_next().await {
            if let Err(err) = res {
                web_error!(" -- 进程处理失败: {:?}", err);
            }
        }
    }

    pub async fn process_one(&self) -> Result<(), AppError> {
        if let WorkFetcher::NoWorkFound = self.process_one_tick_once().await? {
            return Ok(());
        }
        return Ok(());
    }

    async fn process_one_tick_once(&self) -> Result<WorkFetcher, AppError> {
        let work = self.fetch().await?;
        if work.is_none() {
            tokio::task::yield_now().await;
            return Ok(WorkFetcher::NoWorkFound);
        }
        let work = work.unwrap();
        if let Some(worker) = self.workers.get(&work.job.class) {
            let worker = worker.clone();
            match worker.call(work.job.args).await {
                Ok(_) => {}
                Err(err) => {
                    web_error!({
                        "status" = "fail",
                        "class"  = &work.job.class,
                        "queue"  = &work.job.queue,
                        "err"    = format!("{:?}", err)
                    }," -- 进程 {} 处理失败: {:?}", work.job.class, err);
                }
            }
        } else {
            web_error!({
                        "status" = "fail",
                        "class"  = &work.job.class,
                        "queue"  = &work.job.queue,
                    }," -- 进程 {} 未注册", work.job.class);
        }
        Ok(WorkFetcher::Done)
    }
}
