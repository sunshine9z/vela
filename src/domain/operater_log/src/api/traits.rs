use async_trait::async_trait;
use commonx::error::AppError;

use crate::entity::OperaterLog;

#[async_trait]
pub trait OperaterLogDomainTrait {
    async fn create(&self, log: OperaterLog) -> Result<(), AppError>;
}
