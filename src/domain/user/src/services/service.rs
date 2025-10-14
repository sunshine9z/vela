use crate::{
    UserDomainImpl, UserDomainTrait,
    entity::captcha::{CaptchaCacheInfo, CaptchaImage},
};
use captcha_rust::Captcha;
use commonx::error::AppError;
use infrastructurex::web_info;

fn get_cache_key(client_id: &str) -> String {
    format!("capcha:{}", client_id)
}

impl UserDomainTrait for UserDomainImpl {
    async fn gen_captcha(&self, client_id: String, width: u32, height: u32) -> CaptchaImage {
        let captcha = Captcha::new(5, width, height);
        let cache_info = CaptchaCacheInfo {
            client_id: client_id.clone(),
            cache_text: captcha.text.clone(),
        };
        let _ = self
            .cache
            .set_value_ex(&get_cache_key(&client_id), &cache_info, 300)
            .await;
        web_info!("获取验证码:{}", captcha.text);
        CaptchaImage {
            client_id: client_id.clone(),
            image: captcha.base_img,
        }
    }

    async fn get_captcha(&self, client_id: String) -> Result<CaptchaCacheInfo, AppError> {
        self.cache
            .get_oneuse_value::<CaptchaCacheInfo>(&get_cache_key(&client_id))
            .await
            .map_err(|e| match e {
                AppError::CacheNotFoundError(_) => {
                    AppError::CacheNotFoundError(format!("验证码不存在"))
                }
                _ => e,
            })
    }
}
