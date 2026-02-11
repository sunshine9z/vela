use crate::persistence::entities::users::Model as UserModel;
use crate::persistence::id_gen::next_id;
use user_domain::{commons::error::UserDomainError, repository::encrypt::PwdEncryptTrait};

pub struct SysDomainRepositoryImpl {
    pwd_encrypt: Box<dyn PwdEncryptTrait + Sync + Send>,
}

impl SysDomainRepositoryImpl {
    pub fn new(pwd_encrypt: Box<dyn PwdEncryptTrait + Sync + Send>) -> Self {
        Self { pwd_encrypt }
    }
}
