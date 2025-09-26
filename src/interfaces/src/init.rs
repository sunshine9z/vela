use std::{net::SocketAddr, str::FromStr, time::Duration};

use axum::{
    Router,
    middleware::from_fn,
    response::{IntoResponse, Response},
};
use axum_server::tls_rustls::RustlsConfig;
use commonx::error::AppError;
use configx::APP_CONFIG;
use loggerx::web_info;
use tokio::signal::{self, unix::signal};
use tower_http::services::ServeDir;

use crate::{
    API_PATH_PRE,
    middlewares::{request_log::request_log_fn_mid, set_auth_middleware, set_common_middleware},
    routes::routes,
};

pub async fn start_server() -> Result<(), AppError> {
    let server_config = APP_CONFIG.server.clone();
    let addr = format!("{}:{}", server_config.host, server_config.port);
    web_info!(
        "Server is running on {}:{}",
        server_config.host,
        server_config.port
    );

    let router = set_common_middleware(set_routes());
    if server_config.ssl.enable {
        start_https_server(router, &addr).await?;
    } else {
        start_http_server(router, &addr).await?;
    }

    Ok(())
}

fn set_routes() -> Router {
    let server_config = APP_CONFIG.server.clone();
    let static_dir = ServeDir::new(server_config.static_dir);
    // let webdir = ServeDir::new(serverconfig.web_dir);
    Router::new()
        .nest_service("/static", static_dir)
        // .nest_service("/", webdir)
        .nest(API_PATH_PRE, set_auth_middleware(routes()))
        .layer(from_fn(request_log_fn_mid))
}

async fn start_https_server(app: Router, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server_config = APP_CONFIG.server.clone();
    let config = RustlsConfig::from_pem_file(server_config.ssl.cert, server_config.ssl.key)
        .await
        .map_err(|e| format!("Failed to load TLS config: {}", e))?;
    let socket_addr =
        SocketAddr::from_str(addr).map_err(|e| format!("Failed to parse socket address: {}", e))?;

    let handle = axum_server::Handle::new();

    tokio::spawn({
        let handle = handle.clone();
        async move {
            shutdown_signal().await;
            handle.graceful_shutdown(Some(Duration::from_secs(30)));
        }
    });

    tracing::info!("启动https服务: {}", addr);
    axum_server::bind_rustls(socket_addr, config)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .map_err(|e| format!("HTTPS server error: {}", e))?;
    Ok(())
}

async fn start_http_server(app: Router, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind to address: {}", e))?;
    tracing::info!("启动http服务: {}", addr);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| format!("HTTP server error: {}", e))?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    web_info!("signal received, starting graceful shutdown");
}
