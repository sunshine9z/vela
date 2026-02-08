// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use axum::{Extension, response::IntoResponse};
use commonx::error::AppError;
use infrastructurex::persistence::id_gen::next_id;
use userDomain::{
    api::{
        dto::auth::{AuthDto, AuthDtoWithCaptcha},
        traits::UserDomainTrait,
    },
    entity::{captcha::CaptchaImage, user::User},
};

use crate::{
    common::{jwt::authorize, validated_json::VJson, validated_query::VQuery},
    controller::USER_CONTROLLER,
    middlewares::ReqCtx,
    resp::ApiResponse,
    types::{
        auth_jwt::Claims,
        user_info::{
            ClientInfoReq, GetByIdReq, GetByUsernameReq, LoginReq, LoginResp, LoginWithCaptchaReq,
        },
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

    async fn login(&self, _req_ctx: ReqCtx, args: LoginReq) -> Result<LoginResp, AppError> {
        let user = self
            .user_domain
            .login(AuthDto {
                username: args.username,
                password: args.password,
                client_id: args.client_id.unwrap_or_default(),
            })
            .await?;

        let auth_pyload = Claims {
            username: user.username.clone(),
            id: user.id,
            role: user.role_id,
            token_id: next_id(),
            ..Default::default()
        };
        let token = authorize(auth_pyload).await.unwrap();
        Ok(LoginResp {
            token: token.token,
            user: user,
        })
    }
    async fn login_with_captcha(
        &self,
        _req_ctx: ReqCtx,
        args: LoginWithCaptchaReq,
    ) -> Result<LoginResp, AppError> {
        let user = self
            .user_domain
            .login_with_captcha(AuthDtoWithCaptcha {
                username: args.username,
                password: args.password,
                client_id: args.client_id,
                captcha: args.captcha,
            })
            .await?;

        let auth_pyload = Claims {
            username: user.username.clone(),
            id: user.id,
            role: user.role_id,
            token_id: next_id(),
            ..Default::default()
        };
        let token = authorize(auth_pyload).await.unwrap();
        Ok(LoginResp {
            token: token.token,
            user: user,
        })
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

impl<T: UserDomainTrait + Sync + Send> UserController<T> {
    pub fn new(user_domain: T) -> Self {
        Self { user_domain }
    }
}
