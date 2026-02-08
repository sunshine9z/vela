use std::time::Duration;

use commonx::error::AppError;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use commonx::config::APP_CONFIG;
use commonx::web_info;

static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_db() -> Result<(), AppError> {
    let config = &APP_CONFIG.database;
    let mut opt = ConnectOptions::new(&config.uri);
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_millis(config.connect_timeout))
        .idle_timeout(Duration::from_millis(config.idle_timeout))
        .sqlx_logging(config.enable_logging);

    let db = Database::connect(opt).await.unwrap();

    web_info!("数据库连接成功: {}", &config.uri);
    DATABASE.set(db).unwrap();
    Ok(())
}

pub async fn get_db() -> &'static DatabaseConnection {
    DATABASE.get().unwrap()
}
