use std::{net::SocketAddr, str::FromStr, time::Duration};

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use commonx::error::AppError;
use infrastructurex::{config::APP_CONFIG, web_info};
use tokio::signal::{self};

use crate::routes::init_routes;

pub async fn start_server() -> Result<(), AppError> {
    let server_config = APP_CONFIG.server.clone();
    let addr = format!("{}:{}", server_config.host, server_config.port);
    let router = init_routes();
    web_info!("-3.1 加载路由...[ok]");
    
    web_info!(
        "-3.x 启动服务 {}:{}",
        server_config.host,
        server_config.port
    );
    if server_config.ssl.enable {
        start_https_server(router, &addr).await?;
    } else {
        start_http_server(router, &addr).await?;
    }

    Ok(())
}

async fn start_https_server(route: Router, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    web_info!("启动https服务: {}", addr);
    axum_server::bind_rustls(socket_addr, config)
        .handle(handle)
        .serve(route.into_make_service())
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
