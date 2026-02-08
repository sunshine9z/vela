use crate::processor::cron_scheduled::init_corn_schedule;
use crate::processor::processor::Processor;
use crate::processor::wokers::job_worker::JobWorker;
use crate::processor::wokers::mail_worker::MailerWorker;
use crate::processor::worker::AppWorker;
use commonx::config::APP_CONFIG;
use commonx::error::AppError;
use commonx::web_info;

pub const DEFAULT_QUEUE: &[&str] = &["default"];

pub async fn init_worker() -> Result<(), AppError> {
    init_base_worker().await?;
    init_corn_schedule().await?;
    Ok(())
}

pub async fn init_base_worker() -> Result<(), AppError> {
    let worker_config = &APP_CONFIG.workers;
    let queues = gen_queue(&worker_config.queues);
    web_info!(
        queues = ?queues,
        "registering queues (merged config and default)");
    let sched_queues: Vec<String> = match worker_config.sched_queue {
        None => {
            vec![]
        }
        Some(ref qs) => qs.clone(),
    };
    let mut processor = Processor::new(queues, sched_queues, worker_config.num_workers);
    processor.register(JobWorker::new());
    processor.register(MailerWorker::new());

    tokio::spawn(async move {
        processor.run().await;
    });
    Ok(())
}

fn gen_queue(config: &Option<Vec<String>>) -> Vec<String> {
    let mut queues: Vec<_> = DEFAULT_QUEUE.iter().map(|q| q.to_string()).collect();
    match config {
        Some(config) => {
            let _ = config.iter().map(|q| {
                if !queues.contains(q) {
                    queues.push(q.to_string())
                }
            });
            queues
        }
        None => queues,
    }
}
