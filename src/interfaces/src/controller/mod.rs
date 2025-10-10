use once_cell::sync::Lazy;
use userDomain::{UserDomainImpl, new_user_domain};

use crate::controller::user::UserController;

pub mod user;

static MODULE_NAME: &str = "[UserController]";

pub static USER_CONTROLLER: Lazy<UserController<UserDomainImpl>> =
    Lazy::new(|| UserController::new(new_user_domain()));
