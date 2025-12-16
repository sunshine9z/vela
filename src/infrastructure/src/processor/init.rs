use crate::config::APP_CONFIG;
use crate::processor::processor::Processor;
use crate::processor::wokers::job_worker::JobWorker;
use crate::processor::wokers::mail_worker::MailerWorker;
use crate::processor::worker::AppWorker;
use crate::web_info;
use commonx::error::AppError;

pub const DEFAULT_QUEUE: &[&str] = &["default"];

pub async fn worker_init() -> Result<(), AppError> {
    let worker_config = &APP_CONFIG.workers;
    let queues = gen_queue(&worker_config.queues);
    web_info!(
        queues = ?queues,
        "registering queues (merged config and default)");
    let mut processor = Processor::new(queues, worker_config.num_workers);
    processor.register(JobWorker::new());
    processor.register(MailerWorker::new());
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
