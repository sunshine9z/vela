pub mod entity;
mod services;
use captcha_rust::Captcha;
use commonx::error::AppError;
use entity::captcha::CaptchaImage;
use std::{future::Future, sync::Arc};

use infrastructurex::{cache::Cache, web_info};

use crate::entity::captcha::CaptchaCacheInfo;

pub trait UserDomainTrait {
    fn gen_captcha(
        &self,
        client_id: String,
        width: u32,
        height: u32,
    ) -> impl Future<Output = CaptchaImage>;

    fn get_captcha(
        &self,
        client_id: String,
    ) -> impl Future<Output = Result<CaptchaCacheInfo, AppError>>;
}

pub struct UserDomainImpl {
    cache: Arc<Cache>,
}

pub fn new_user_domain(cache: Arc<Cache>) -> UserDomainImpl {
    UserDomainImpl { cache }
}
