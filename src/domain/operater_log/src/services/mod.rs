use async_trait::async_trait;
use commonx::error::AppError;

use crate::{OperaterLogDomainImpl, api::traits::OperaterLogDomainTrait, entity::OperaterLog};

#[async_trait]
impl OperaterLogDomainTrait for OperaterLogDomainImpl {
    async fn create(&self, log: OperaterLog) -> Result<(), AppError> {
        self.repo.create(log).await
    }
}
