use axum::{
    response::IntoResponse,
    routing::{get, post},
};
use infrastructurex::web_info;

use crate::{
    controller::auth::gen_captcha,
    resp::ApiResponse,
    routes::router_group::{RouterGroup, WebPathMethod},
};

// 系统路由
pub fn router_sys() -> RouterGroup {
    RouterGroup::new().nest("/sys", RouterGroup::new().nest("/cache", sys_cache()))
}

// 系统缓存路由
fn sys_cache() -> RouterGroup {
    RouterGroup::new()
        .route(
            "/list",
            WebPathMethod::Get,
            Some("获取缓存列表"),
            get(|| async { "sys cache list" }),
        )
        .route(
            "/clear",
            WebPathMethod::Post,
            Some("清空缓存"),
            post(|| async { "sys cache clear" }),
        )
}

// 白名单路由
pub fn router_sys_white() -> RouterGroup {
    RouterGroup::new()
        .nest(
            "/sys",
            RouterGroup::new().route(
                "/health",
                WebPathMethod::Get,
                Some("系统健康检查"),
                get(health),
            ),
        )
        .nest(
            "/auth",
            RouterGroup::new()
                .route(
                    "/login",
                    WebPathMethod::Post,
                    Some("用户登录"),
                    post(|| async { "auth login" }),
                )
                .route(
                    "/get_captcha",
                    WebPathMethod::Get,
                    Some("获取验证码"),
                    get(gen_captcha),
                ),
        )
}
// 系统健康检查
pub async fn health() -> impl IntoResponse {
    web_info!("sys health check");
    ApiResponse::ok()
}
