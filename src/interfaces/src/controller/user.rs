// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use axum::response::IntoResponse;
use commonx::validated_query::VQuery;
use userDomain::{UserDomainTrait, entity::captcha::CaptchaImage};

use crate::{controller::USER_CONTROLLER, resp::ApiResponse, types::user_info::ClientInfo};

pub async fn get_captcha(VQuery(arg): VQuery<ClientInfo>) -> impl IntoResponse {
    ApiResponse::ok_with_data(USER_CONTROLLER.gen_captcha(arg).await)
}

pub trait UserControllerTrait {
    async fn gen_captcha(&self, client_info: ClientInfo) -> CaptchaImage;
}

pub struct UserController<T: UserDomainTrait> {
    user_domain: T,
}

impl<T: UserDomainTrait> UserControllerTrait for UserController<T> {
    async fn gen_captcha(&self, client_id: ClientInfo) -> CaptchaImage {
        let width = client_id.width.unwrap_or(100);
        let height = client_id.height.unwrap_or(40);
        self.user_domain
            .gen_captcha(client_id.client_id, width, height)
            .await
    }
}

impl<T: UserDomainTrait> UserController<T> {
    pub fn new(user_domain: T) -> Self {
        Self { user_domain }
    }
}
