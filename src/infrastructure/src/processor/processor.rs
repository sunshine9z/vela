use crate::cache::CacheManager;
use crate::processor::job::Job;
use crate::processor::periodic::PeriodicJob;
use crate::processor::unit_of_work::UnitOfWork;
use crate::processor::worker::{Worker, WorkerRef};
use commonx::error::AppError;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Processor {
    queues: Vec<String>,
    periodic_jobs: Vec<PeriodicJob>,
    workers: BTreeMap<String, Arc<WorkerRef>>,
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
            num_workers,
        }
    }

    async fn fetch(&mut self) -> Result<Option<UnitOfWork>, AppError> {
        let cache = CacheManager::instance();
        let res: Option<(String, String)> = cache.brpop(&self.queues, 2).await?;
        if let Some((queue, job_raw)) = res {
            let job: Job = serde_json::from_str(&job_raw)?;
            return Ok(Some(UnitOfWork { queue, job }));
        }
        Ok(None)
    }

    pub fn register<W: Worker + 'static>(&mut self, worker: W) {
        self.workers
            .insert(W::class_name(), Arc::new(WorkerRef::new(worker)));
    }
}
