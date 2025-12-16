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
    pub async fn init_all(&self) -> Result<(), UserDomainError> {
        let password = self.pwd_encrypt.encrypt(&"123456".to_string())?;
        let users = vec![user_domain::entity::user::User {
            id: next_id(),
            name: Some("admin".to_string()),
            username: "admin".to_string(),
            password,
            ..Default::default()
        }];
        for user in users {
            // 这里可以添加对 user 的处理逻辑，比如保存到数据库等
            UserModel::create(user).await.map_or_else(
                |e| Err(UserDomainError::DbError(e.to_string())),
                |ret| Ok(ret),
            )?;
        }
        Ok(())
    }
}
