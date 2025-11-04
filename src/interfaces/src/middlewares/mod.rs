mod auth;
mod operate_log;
pub mod request_log;

use std::{any::Any, time::Duration};

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
    http::HeaderName,
    middleware,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use infrastructurex::{config::APP_CONFIG, web_error, web_info};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::{CompressionLayer, DefaultPredicate, Predicate, predicate::NotForContentType},
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    timeout::TimeoutLayer,
};

use crate::{
    middlewares::{auth::check_permission_mid, operate_log::operate_log_fn_mid},
    types::user_info::UserInfo,
};

const MIDDLEWARE_NAME: &str = "[Middleware]";

#[derive(Debug, Clone, Default)]
pub struct ReqCtx {
    pub ip: String,
    pub ori_uri: String,
    pub path: String,
    pub path_params: String,
    pub method: String,
    // pub user_agent: String,
}

pub fn set_no_auth_middleware(router: Router) -> Router {
    router
}

pub fn set_auth_middleware(router: Router) -> Router {
    router
        .layer(middleware::from_fn(operate_log_fn_mid))
        .layer(middleware::from_fn(check_permission_mid))
        // .layer(middleware::from_fn(req_info_fn_mid)) // 注入请求信息
        .layer(middleware::from_extractor::<UserInfo>()) //从token中注入用户信息
}

pub fn set_common_middleware(mut router: Router) -> Router {
    let server_config = &APP_CONFIG.server;

    // payload 限制
    if let Some(limit) = server_config.middlewares.limit_payload.as_ref() {
        if let Ok(size) = byte_unit::Byte::parse_str(&limit, true) {
            // 1. 禁用默认请求体限制，改用自定义限制（10MB = 10 * 1024 * 1024 字节）
            router = router
                .layer(DefaultBodyLimit::disable())
                .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024));
            web_info!("{MIDDLEWARE_NAME} 添加payload限制{:?}", size);
        }
    }
    // CORS配置
    router = router.layer(
        CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any),
    );
    // Panic处理
    router = router.layer(CatchPanicLayer::custom(handle_panic));
    // 压缩
    if let Some(compression) = server_config.middlewares.compression.clone() {
        if compression.enable {
            let predicate =
                DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
            router = router.layer(CompressionLayer::new().compress_when(predicate));
            web_info!("{MIDDLEWARE_NAME} 添加压缩中间件");
        }
    }
    // 超时
    if let Some(time_request) = server_config.middlewares.timeout_request.as_ref() {
        if time_request.enable {
            router = router.layer(TimeoutLayer::new(Duration::from_millis(
                time_request.timeout,
            )));
            web_info!("{MIDDLEWARE_NAME} 添加超时{}ms中间件", time_request.timeout);
        }
    }
    // 需要设置一个请求头的键名，一般叫x-request-id
    router = router.layer(SetRequestIdLayer::new(
        HeaderName::from_static("x-request-id"),
        MakeRequestUuid,
    ));

    router
}

pub fn parse_ip(req: &Request) -> String {
    req.headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|ip| {
            ip.split(',')
                .next()
                .unwrap_or("unknown IP")
                .trim()
                .to_string()
        })
        .unwrap_or_else(|| "unknown IP".to_string())
}

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let err = err.downcast_ref::<String>().map_or_else(
        || err.downcast_ref::<&str>().map_or("no error details", |s| s),
        |s| s.as_str(),
    );
    web_error!(err, "server_panic");
    (StatusCode::INTERNAL_SERVER_ERROR, "服务器错误".to_string()).into_response()
}
