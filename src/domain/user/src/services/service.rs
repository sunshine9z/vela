use crate::{
    MODEL_USER_DOMAIN, UserDomainImpl,
    api::{dto::auth::AuthDto, traits::UserDomainTrait},
    commons::error::UserDomainError,
    entity::{
        self,
        captcha::{CaptchaCacheInfo, CaptchaImage},
    },
};
use async_trait::async_trait;
use captcha_rust::Captcha;
use tracing::info;

fn get_cache_key(client_id: &str) -> String {
    format!("capcha:{}", client_id)
}

#[async_trait]
impl UserDomainTrait for UserDomainImpl {
    async fn gen_captcha(
        &self,
        client_id: String,
        width: u32,
        height: u32,
    ) -> Result<CaptchaImage, UserDomainError> {
        let captcha = Captcha::new(5, width, height);
        self.cache
            .set_captcha(
                get_cache_key(&client_id),
                CaptchaCacheInfo {
                    client_id: client_id.clone(),
                    cache_text: captcha.text.clone(),
                },
            )
            .await?;
        info!(target: MODEL_USER_DOMAIN,"生成验证码:{} -> {}", client_id, captcha.text);
        Ok(CaptchaImage {
            client_id: client_id,
            image: captcha.base_img,
        })
    }

    async fn get_captcha(&self, client_id: String) -> Result<CaptchaCacheInfo, UserDomainError> {
        self.cache.get_captcha(get_cache_key(&client_id)).await
    }

    async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<entity::user::User>, UserDomainError> {
        self.user_repo
            .get_by_username(username)
            .await
            .map_err(|e| UserDomainError::DbError(e.to_string()))
    }

    async fn login(&self, auth_req: AuthDto) -> Result<entity::user::User, UserDomainError> {
        let captcha_info = self
            .cache
            .get_captcha(get_cache_key(&auth_req.client_id))
            .await?;
        info!(target: MODEL_USER_DOMAIN,
            "获取验证码:{}:{}",
            auth_req.client_id.clone(),
            captcha_info.cache_text
        );

        if captcha_info.client_id != auth_req.client_id {
            return Err(UserDomainError::AuthError(format!(
                "获取的验证码client_id与登录请求client_id不一致"
            )));
        }

        if captcha_info.cache_text.to_lowercase() != auth_req.captcha.to_lowercase() {
            return Err(UserDomainError::AuthError(format!("验证码错误")));
        }
        let user = self
            .user_repo
            .get_by_username(auth_req.username.clone())
            .await?;

        if user.is_none() {
            return Err(UserDomainError::AuthError(format!("用户不存在")));
        }

        let user = user.unwrap();

        if !self
            .pwd_encrypt
            .verify(auth_req.password.clone(), user.password.clone())
        {
            return Err(UserDomainError::AuthError(format!("密码错误")));
        }
        Ok(user)
    }
}
