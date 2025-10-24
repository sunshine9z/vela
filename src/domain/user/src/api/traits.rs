use crate::{
    api::dto::auth::AuthDto,
    commons::error::UserDomainError,
    entity::{
        self,
        captcha::{CaptchaCacheInfo, CaptchaImage},
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait UserDomainTrait {
    async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<entity::user::User>, UserDomainError>;
    async fn gen_captcha(
        &self,
        client_id: String,
        width: u32,
        height: u32,
    ) -> Result<CaptchaImage, UserDomainError>;
    async fn get_captcha(&self, client_id: String) -> Result<CaptchaCacheInfo, UserDomainError>;
    async fn login(&self, auth_req: AuthDto) -> Result<entity::user::User, UserDomainError>;
}
