use crate::{commons::error::UserDomainError, entity::captcha::CaptchaCacheInfo};
use async_trait::async_trait;

#[async_trait]
pub trait CacheRepositoryTrait {
    async fn set_captcha(
        &self,
        key: String,
        captcha: CaptchaCacheInfo,
    ) -> Result<bool, UserDomainError>;

    async fn get_captcha(&self, client_id: String) -> Result<CaptchaCacheInfo, UserDomainError>;
}
