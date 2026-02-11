use axum::{
    response::IntoResponse,
    routing::{get, post},
};
use commonx::web_info;

use crate::{
    controller,
    resp::ApiResponse,
    routes::router_group::{RouterGroup, WebPathMethod},
};

// 系统路由
pub fn router_sys() -> RouterGroup {
    RouterGroup::new()
        .nest(
            "/sys",
            RouterGroup::new().nest("/cache", sys_cache()).route(
                "/init_all",
                WebPathMethod::Post,
                Some("初始化数据库"),
                post(controller::sys::init_all),
            ),
        )
        .nest(
            "/cornJob",
            RouterGroup::new()
                .route(
                    "/list",
                    WebPathMethod::Get,
                    Some("获取CornJob列表"),
                    get(controller::corn_job::list),
                )
                .route(
                    "/create",
                    WebPathMethod::Post,
                    Some("创建CornJob"),
                    post(controller::corn_job::create),
                )
                .route(
                    "/delete",
                    WebPathMethod::Post,
                    Some("删除CornJob"),
                    post(controller::corn_job::delete_by_id),
                )
                .route(
                    "/update",
                    WebPathMethod::Post,
                    Some("更新CornJob"),
                    post(controller::corn_job::update_by_id),
                )
                .route(
                    "/get_by_id",
                    WebPathMethod::Get,
                    Some("根据ID获取CornJob"),
                    get(controller::corn_job::get_by_id),
                ),
        )
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
                    post(controller::user::login),
                )
                .route(
                    "/login_with_captcha",
                    WebPathMethod::Post,
                    Some("用户登录（验证码）"),
                    post(controller::user::login_with_captcha),
                )
                .route(
                    "/get_captcha",
                    WebPathMethod::Get,
                    Some("获取验证码"),
                    get(controller::user::get_captcha),
                )
                .route(
                    "/get_by_username",
                    WebPathMethod::Get,
                    Some("根据用户名获取用户"),
                    get(controller::user::get_by_username),
                )
                .route(
                    "/get_by_id",
                    WebPathMethod::Get,
                    Some("根据ID获取用户"),
                    get(controller::user::get_by_id),
                ),
        )
}

// 系统健康检查
pub async fn health() -> impl IntoResponse {
    web_info!("sys health check");
    ApiResponse::ok()
}
