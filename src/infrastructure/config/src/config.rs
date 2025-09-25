use commonx::error::AppError;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use serde_yaml::from_str;
use std::{env, fs};

static CONFIG_PATH: &str = "src/conf";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub logger: Logger,
    pub cache: CacheConfig,
    pub snowgenera: SnowGenerator,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn init() -> Self {
        let env: String = env::var("environment").unwrap_or_else(|_| "dev".to_string());
        Self::load_config(format!("config.{}", env)).unwrap()
    }
    fn load_config(env: String) -> Result<Self, AppError> {
        // 这里应该继续实现加载配置文件的逻辑
        let file_path = env::current_dir()
            .unwrap()
            .join(CONFIG_PATH)
            .join(format!("{env}.yaml"));
        println!("加载的配置文件路径: {}", file_path.display());
        let content = fs::read_to_string(&file_path).map_err(|e| {
            AppError::ConfigError(format!("读取配置文件:{},错误: {}", file_path.display(), e))
        })?;

        let conf = from_str::<Config>(&content)
            .map_err(|e| AppError::ConfigError(format!("解析配置文件:{},错误: {}", &content, e)))?;
        Ok(conf)
    }
}

// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub static_dir: String,
    pub web_dir: String,
    pub upload_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Logger {
    pub enable: bool,
    pub level: LogLevel,
    pub format: LogFormat,
    pub log_dir: String,
    pub web_file_name: String,
    pub api_file_name: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    #[default]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        to_variant_name(self).expect("日志level错误").fmt(f)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum LogFormat {
    #[serde(rename = "compact")]
    #[default]
    Compact,
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "json")]
    Json,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub cache_type: String,
    pub namespace: Option<String>,
    pub pool_size: Option<u32>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SnowGenerator {
    pub machine_id: i32,
    pub node_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub uri: String,
    #[serde(default = "default_true")]
    pub enable_logging: bool,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub min_connections: u32,
    pub max_connections: u32,
    #[serde(default = "default_true")]
    pub auto_migrate: bool,
    #[serde(default = "default_false")]
    pub dangerously_truncate: bool,
    #[serde(default = "default_false")]
    pub dangerously_recreate: bool,
}

fn default_false() -> bool {
    false
}
fn default_true() -> bool {
    true
}
