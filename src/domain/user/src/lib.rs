pub mod entity;
use captcha_rust::Captcha;
use entity::captcha::CaptchaImage;
use std::future::Future;

use infrastructurex::{cache::CacheManager, web_info};

use crate::entity::captcha::CaptchaCacheInfo;

pub trait UserDomainTrait {
    fn gen_captcha(
        &self,
        client_id: String,
        width: u32,
        height: u32,
    ) -> impl Future<Output = CaptchaImage>;
}

pub struct UserDomainImpl {}

impl UserDomainTrait for UserDomainImpl {
    async fn gen_captcha(&self, client_id: String, width: u32, height: u32) -> CaptchaImage {
        let captcha = Captcha::new(5, width, height);
        let cacheinfo = CaptchaCacheInfo {
            client_id: client_id.clone(),
            cache_text: captcha.text.clone(),
        };
        let cache = CacheManager::instance().await;
        let _ = cache
            .set_value_ex(&format!("capcha:{}", client_id), &cacheinfo, 300)
            .await;
        web_info!("获取验证码:{}", captcha.text);
        CaptchaImage {
            image: captcha.base_img,
            code: captcha.text,
        }
    }
}

pub fn new_user_domain() -> UserDomainImpl {
    UserDomainImpl {}
}
