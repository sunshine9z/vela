pub mod api;
pub mod commons;
pub mod entity;
pub mod repository;
mod services;

pub const MODEL_USER_DOMAIN: &str = "userDomain";

use crate::repository::{
    cache::CacheRepositoryTrait, encrypt::PwdEncryptTrait, user::UserRepositoryTrait,
};

pub struct UserDomainImpl {
    cache: Box<dyn CacheRepositoryTrait + Sync + Send>,
    user_repo: Box<dyn UserRepositoryTrait + Sync + Send>,
    pwd_encrypt: Box<dyn PwdEncryptTrait + Sync + Send>,
}

pub fn new_user_domain(
    cache: Box<dyn CacheRepositoryTrait + Sync + Send>,
    user_repo: Box<dyn UserRepositoryTrait + Sync + Send>,
    pwd_encrypt: Box<dyn PwdEncryptTrait + Sync + Send>,
) -> UserDomainImpl {
    UserDomainImpl {
        cache,
        user_repo,
        pwd_encrypt,
    }
}
