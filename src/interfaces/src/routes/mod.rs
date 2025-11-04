mod router_group;
mod sys;
use axum::{Router, middleware::from_fn, response::IntoResponse};
use infrastructurex::config::APP_CONFIG;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::{
    API_PATH_PRE,
    middlewares::{
        request_log::request_log_fn_mid, set_auth_middleware, set_common_middleware,
        set_no_auth_middleware,
    },
    resp::ApiResponse,
    routes::sys::{router_sys, router_sys_white},
};

// static MODULE_NAME: &str = "[routes]";

fn routes() -> Router {
    router_sys().into()
}

// 白名单路由
fn white_routers() -> Router {
    router_sys_white().into()
}

fn set_routes() -> Router {
    let server_config = &APP_CONFIG.server;
    let static_dir = ServeDir::new(&server_config.static_dir);
    // let webdir = ServeDir::new(serverconfig.web_dir);
    Router::new()
        .nest_service("/static", static_dir)
        // .nest_service("/", webdir)
        .nest(API_PATH_PRE, set_no_auth_middleware(white_routers()))
        .nest(API_PATH_PRE, set_auth_middleware(routes()))
        .layer(from_fn(request_log_fn_mid))
        // 3. 请求跟踪日志（记录请求详情，便于排查问题）
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true)) // 记录请求头
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO)) // 请求开始日志
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)) // 响应日志（含耗时）
                .on_failure(DefaultOnFailure::new().level(tracing::Level::ERROR)), // 失败日志
        )
        .with_state(()) // Axum 0.8+ 必需：明确状态（空状态用 ()）
        .fallback(handle_404)
}

pub fn init_routes() -> Router {
    set_common_middleware(set_routes())
}

async fn handle_404() -> impl IntoResponse {
    ApiResponse::not_found("页面不存在或参数错误")
}
