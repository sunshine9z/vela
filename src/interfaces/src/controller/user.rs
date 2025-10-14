// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use axum::response::IntoResponse;
use commonx::error::AppError;
use hyper::HeaderMap;
use infrastructurex::web_info;
use userDomain::{UserDomainTrait, entity::captcha::CaptchaImage};

use crate::{
    MODULE_NAME,
    common::{validated_json::VJson, validated_query::VQuery},
    controller::USER_CONTROLLER,
    resp::ApiResponse,
    types::user_info::{ClientInfo, LoginReq, LoginResp},
};

pub async fn get_captcha(VQuery(arg): VQuery<ClientInfo>) -> impl IntoResponse {
    ApiResponse::ok_with_data(USER_CONTROLLER.gen_captcha(arg).await)
}

pub async fn login(header: HeaderMap, VJson(arg): VJson<LoginReq>) -> impl IntoResponse {
    match USER_CONTROLLER.login(header, arg).await {
        Ok(resp) => ApiResponse::ok_with_data(resp),
        Err(err) => err.into_response(),
    }
}

pub trait UserControllerTrait {
    async fn gen_captcha(&self, client_info: ClientInfo) -> CaptchaImage;
    async fn login(&self, header: HeaderMap, args: LoginReq) -> Result<LoginResp, AppError>;
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
    async fn login(&self, header: HeaderMap, args: LoginReq) -> Result<LoginResp, AppError> {
        let captcha_info = self.user_domain.get_captcha(args.client_id.clone()).await?;
        web_info!(
            "{MODULE_NAME}获取验证码:{}:{}",
            args.client_id.clone(),
            captcha_info.cache_text
        );

        // let req_ctx = match req.extensions().get::<ReqCtx>() {
        //     Some(ctx) => ctx.clone(),
        //     None => return Ok(next.run(req).await),
        // };

        Ok(LoginResp {
            token: "".to_string(),
        })
    }
}

impl<T: UserDomainTrait> UserController<T> {
    pub fn new(user_domain: T) -> Self {
        Self { user_domain }
    }
}
