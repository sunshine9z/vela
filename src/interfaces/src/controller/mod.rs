use infrastructurex::{
    container::{sys_domain::SysDomainRepositoryImpl, user_domain::new_user_domain_service},
    encrypt::pwd_encrypt::PwdEncryptImpl,
};
use once_cell::sync::Lazy;
use userDomain::UserDomainImpl;

use crate::controller::{sys::SysController, user::UserController};

pub mod sys;
pub mod user;

// static MODULE_NAME: &str = "[UserController]";
// static SYS_MODULE_NAME: &str = "[SysController]";

pub static USER_CONTROLLER: Lazy<UserController<UserDomainImpl>> =
    Lazy::new(|| UserController::new(new_user_domain_service()));

pub static SYS_CONTROLLER: Lazy<SysController> =
    Lazy::new(|| SysController::new(SysDomainRepositoryImpl::new(Box::new(PwdEncryptImpl {}))));
