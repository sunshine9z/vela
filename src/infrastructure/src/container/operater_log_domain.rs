use async_trait::async_trait;
use commonx::error::AppError;
use operater_log_domain::{
    OperaterLogDomainImpl, entity::OperaterLog, new_operater_log_domain,
    repository::OperaterLogRepositoryTrait,
};

pub struct OperaterLogRepositoryImpl {}

#[async_trait]
impl OperaterLogRepositoryTrait for OperaterLogRepositoryImpl {
    async fn create(&self, log: OperaterLog) -> Result<(), AppError> {
        Ok(())
    }
}

pub fn new_operater_log_domain_service() -> OperaterLogDomainImpl {
    new_operater_log_domain(Box::new(OperaterLogRepositoryImpl {}))
}
