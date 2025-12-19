use async_trait::async_trait;
use commonx::error::AppError;
use serde::{Deserialize, Serialize};

use crate::{
    processor::worker::{AppWorker, Worker},
    web_info,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Email {
    pub from: Option<String>,
    pub to: String,
    pub text: String,
}

pub struct MailerWorker {}

#[async_trait]
impl AppWorker for MailerWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker for MailerWorker {
    async fn perform(&self, args: serde_json::Value) -> Result<(), AppError> {
        let email = serde_json::from_value::<Email>(args)?;
        web_info!(" -- 发送邮件: {:?}", email);
        Ok(())
    }
}
