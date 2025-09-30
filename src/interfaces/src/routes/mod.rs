mod router_group;
mod sys;
use axum::{Router, middleware::from_fn, response::IntoResponse};
use infrastructurex::config::APP_CONFIG;
use tower_http::services::ServeDir;

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
    let server_config = APP_CONFIG.server.clone();
    let static_dir = ServeDir::new(server_config.static_dir);
    // let webdir = ServeDir::new(serverconfig.web_dir);
    Router::new()
        .nest_service("/static", static_dir)
        // .nest_service("/", webdir)
        .nest(API_PATH_PRE, set_no_auth_middleware(white_routers()))
        .nest(API_PATH_PRE, set_auth_middleware(routes()))
        .fallback(handle_404)
        .layer(from_fn(request_log_fn_mid))
}

pub fn init_routes() -> Router {
    set_common_middleware(set_routes())
}

async fn handle_404() -> impl IntoResponse {
    ApiResponse::not_found("页面不存在或参数错误")
}
