use crate::processor::worker::{AppWorker, Worker};
use async_trait::async_trait;
use commonx::error::AppError;
use commonx::web_info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct JobMsg {
    pub job_id: i64,
}

pub struct JobWorker {}

impl AppWorker for JobWorker {
    fn new() -> Self {
        JobWorker {}
    }
}

#[async_trait]
impl Worker for JobWorker {
    async fn perform(&self, args: serde_json::Value) -> Result<(), AppError> {
        let job_msg: JobMsg = serde_json::from_value(args)?;
        web_info!(" -- JobWorker: {:?}", job_msg);
        Ok(())
    }
}
