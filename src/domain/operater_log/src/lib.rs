use crate::repository::OperaterLogRepositoryTrait;

pub mod api;
pub mod entity;
pub mod repository;
pub mod services;

pub const MODEL_OPERATOR_LOG: &str = "operatorLog";

pub struct OperaterLogDomainImpl {
    repo: Box<dyn OperaterLogRepositoryTrait + Sync + Send>,
}

pub fn new_operater_log_domain(
    repo: Box<dyn OperaterLogRepositoryTrait + Sync + Send>,
) -> OperaterLogDomainImpl {
    OperaterLogDomainImpl { repo }
}
