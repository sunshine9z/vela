use crate::persistence::entities::sys_oper_log::Model as OperaterLogModel;
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
        OperaterLogModel::create(
            log.api_name,
            log.request_method,
            log.oper_id,
            log.oper_name,
            log.oper_url,
            log.oper_ip,
            log.oper_location,
            log.oper_param,
            log.json_result,
            log.cost_time,
        )
        .await
        .map_err(|e| AppError::from(e))?;
        Ok(())
    }
}

pub fn new_operater_log_domain_service() -> OperaterLogDomainImpl {
    new_operater_log_domain(Box::new(OperaterLogRepositoryImpl {}))
}
