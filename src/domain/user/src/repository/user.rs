use async_trait::async_trait;

use crate::commons::error::UserDomainError;
use crate::entity::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_by_username(&self, username: String) -> Result<Option<User>, UserDomainError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, UserDomainError>;
    async fn update_by_id(&self, id: i64, user: User) -> Result<(), UserDomainError>;
    async fn create(&self, user: User) -> Result<i64, UserDomainError>;
    async fn remove(&self, id: i64) -> Result<(), UserDomainError>;
}
