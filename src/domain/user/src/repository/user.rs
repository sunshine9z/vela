use async_trait::async_trait;

use crate::commons::error::UserDomainError;
use crate::entity::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_by_username(&self, username: String) -> Result<Option<User>, UserDomainError>;
}
