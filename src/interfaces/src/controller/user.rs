// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use std::time::Instant;

use axum::{Extension, response::IntoResponse};
use chrono::Local;
use commonx::error::AppError;
use infrastructurex::persistence::id_gen::next_id;
use operaterLogDomain::{api::traits::OperaterLogDomainTrait, entity::OperaterLog};
use userDomain::{
    api::{
        dto::{
            auth::{AuthDto, AuthDtoWithCaptcha},
            user_info::UserInfoDto,
        },
        traits::UserDomainTrait,
    },
    entity::{captcha::CaptchaImage, user::User},
};

use crate::{
    common::{OPERATOR_LOG_DOMAIN, jwt::authorize, validated_json::VJson, validated_query::VQuery},
    controller::USER_CONTROLLER,
    middlewares::ReqCtx,
    resp::ApiResponse,
    types::{
        GetByIdReq,
        auth_jwt::Claims,
        user_info::{ClientInfoReq, GetByUsernameReq, LoginReq, LoginResp, LoginWithCaptchaReq},
    },
};

pub async fn get_captcha(VQuery(arg): VQuery<ClientInfoReq>) -> impl IntoResponse {
    ApiResponse::from_result(USER_CONTROLLER.gen_captcha(arg).await)
}

pub async fn login(
    Extension(req_ctx): Extension<ReqCtx>,
    VJson(arg): VJson<LoginReq>,
) -> impl IntoResponse {
    ApiResponse::from_result(USER_CONTROLLER.login(req_ctx, arg).await)
}

pub async fn login_with_captcha(
    Extension(req_ctx): Extension<ReqCtx>,
    VJson(arg): VJson<LoginWithCaptchaReq>,
) -> impl IntoResponse {
    ApiResponse::from_result(USER_CONTROLLER.login_with_captcha(req_ctx, arg).await)
}

pub async fn get_by_username(VQuery(arg): VQuery<GetByUsernameReq>) -> impl IntoResponse {
    ApiResponse::from_result(USER_CONTROLLER.get_by_username(arg.username).await)
}

pub async fn get_by_id(VQuery(arg): VQuery<GetByIdReq>) -> impl IntoResponse {
    ApiResponse::from_result(USER_CONTROLLER.get_by_id(arg.id).await)
}

pub trait UserControllerTrait {
    async fn gen_captcha(&self, client_info: ClientInfoReq) -> Result<CaptchaImage, AppError>;
    async fn login_with_captcha(
        &self,
        req_ctx: ReqCtx,
        args: LoginWithCaptchaReq,
    ) -> Result<LoginResp, AppError>;
    async fn login(&self, req_ctx: ReqCtx, args: LoginReq) -> Result<LoginResp, AppError>;
    async fn get_by_username(&self, username: String) -> Result<Option<User>, AppError>;
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
}

pub struct UserController<T: UserDomainTrait + Sync + Send> {
    user_domain: T,
}

impl<T: UserDomainTrait + Sync + Send> UserControllerTrait for UserController<T> {
    async fn gen_captcha(&self, client_id: ClientInfoReq) -> Result<CaptchaImage, AppError> {
        let width = client_id.width.unwrap_or(100);
        let height = client_id.height.unwrap_or(40);
        self.user_domain
            .gen_captcha(client_id.client_id, width, height)
            .await
            .map_err(|e| e.into())
    }

    async fn login(&self, req_ctx: ReqCtx, args: LoginReq) -> Result<LoginResp, AppError> {
        let start_time = Instant::now();
        let user = self
            .user_domain
            .login(AuthDto {
                username: args.username,
                password: args.password,
                client_id: args.client_id.unwrap_or_default(),
            })
            .await?;

        do_login(req_ctx, user, start_time).await
    }
    async fn login_with_captcha(
        &self,
        req_ctx: ReqCtx,
        args: LoginWithCaptchaReq,
    ) -> Result<LoginResp, AppError> {
        let start_time = Instant::now();
        let user = self
            .user_domain
            .login_with_captcha(AuthDtoWithCaptcha {
                username: args.username,
                password: args.password,
                client_id: args.client_id,
                captcha: args.captcha,
            })
            .await?;

        do_login(req_ctx, user, start_time).await
    }
    async fn get_by_username(&self, username: String) -> Result<Option<User>, AppError> {
        self.user_domain
            .get_by_username(username)
            .await
            .map_err(|e| e.into())
    }
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        self.user_domain.get_by_id(id).await.map_err(|e| e.into())
    }
}

async fn do_login(
    req_ctx: ReqCtx,
    user: UserInfoDto,
    start_time: Instant,
) -> Result<LoginResp, AppError> {
    let auth_pyload = Claims {
        username: user.username.clone(),
        id: user.id,
        role: user.role_id,
        token_id: next_id(),
        ..Default::default()
    };
    let token = authorize(auth_pyload).await?;
    let res = LoginResp {
        token: token.token,
        user: user,
    };
    // 记录操作日志
    OPERATOR_LOG_DOMAIN
        .create(OperaterLog {
            id: next_id(),
            api_name: req_ctx.path.clone(),
            oper_ip: req_ctx.ip.clone(),
            oper_id: res.user.id,
            oper_name: res.user.username.clone(),
            oper_url: req_ctx.ori_uri.clone(),
            oper_location: req_ctx.ori_uri.clone(),
            request_method: req_ctx.method.clone(),
            oper_param: req_ctx.path_params.clone(),
            json_result: serde_json::to_string(&res).unwrap_or_default(),
            cost_time: start_time.elapsed().as_millis() as i64,
            oper_time: Local::now(),
            ..Default::default()
        })
        .await?;

    Ok(res)
}

impl<T: UserDomainTrait + Sync + Send> UserController<T> {
    pub fn new(user_domain: T) -> Self {
        Self { user_domain }
    }
}
