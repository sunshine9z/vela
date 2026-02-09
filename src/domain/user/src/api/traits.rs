use crate::{
    api::dto::{
        auth::{AuthDto, AuthDtoWithCaptcha},
        user_info::UserInfoDto,
    },
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
    async fn get_by_id(&self, id: i64) -> Result<Option<entity::user::User>, UserDomainError>;
    async fn gen_captcha(
        &self,
        client_id: String,
        width: u32,
        height: u32,
    ) -> Result<CaptchaImage, UserDomainError>;
    async fn get_captcha(&self, client_id: String) -> Result<CaptchaCacheInfo, UserDomainError>;
    async fn login(&self, auth_req: AuthDto) -> Result<UserInfoDto, UserDomainError>;
    async fn login_with_captcha(
        &self,
        auth_req: AuthDtoWithCaptcha,
    ) -> Result<UserInfoDto, UserDomainError>;
}
