use infrastructurex::container::operater_log_domain::new_operater_log_domain_service;
use once_cell::sync::Lazy;
use operaterLogDomain::OperaterLogDomainImpl;

pub mod jwt;
mod validated_form;
pub mod validated_json;
pub mod validated_query;

pub static OPERATOR_LOG_DOMAIN: Lazy<OperaterLogDomainImpl> =
    Lazy::new(|| new_operater_log_domain_service());
