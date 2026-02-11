use infrastructurex::container::{
    job_domain::{new_job_domain_service, new_job_query_service},
    user_domain::new_user_domain_service,
};
use jobDomain::JobDomainImpl;
use once_cell::sync::Lazy;
use queryx::corn_job::services::JobQueryImpl;
use userDomain::UserDomainImpl;

use crate::controller::{corn_job::CornJobController, sys::SysController, user::UserController};

pub mod corn_job;
pub mod sys;
pub mod user;

// static MODULE_NAME: &str = "[UserController]";
// static SYS_MODULE_NAME: &str = "[SysController]";

pub static USER_CONTROLLER: Lazy<UserController<UserDomainImpl>> =
    Lazy::new(|| UserController::new(new_user_domain_service()));

pub static SYS_CONTROLLER: Lazy<SysController> = Lazy::new(|| SysController::new());

pub static CORN_JOB_CONTROLLER: Lazy<CornJobController<JobDomainImpl, JobQueryImpl>> =
    Lazy::new(|| CornJobController::new(new_job_domain_service(), new_job_query_service()));
