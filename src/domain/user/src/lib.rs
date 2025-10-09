pub mod entity;
use captcha_rust::Captcha;
use entity::captcha::CaptchaImage;

use async_trait::async_trait;
use infrastructurex::{cache::CacheManager, web_info};

use crate::entity::captcha::CaptchaCacheInfo;

#[async_trait]
pub trait UserDomain {
    async fn gen_captcha(client_id: String) -> CaptchaImage;
}

struct UserDomainImpl {}

#[async_trait]
impl UserDomain for UserDomainImpl {
    async fn gen_captcha(client_id: String) -> CaptchaImage {
        let captcha = Captcha::new(5, 130, 40);

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

pub fn new_user_domain() -> impl UserDomain {
    UserDomainImpl {}
}
