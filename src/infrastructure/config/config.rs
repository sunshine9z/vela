use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use serde_yaml::from_str;
use std::{env, fs};

use crate::common::error::AppError;

static CONFIG_PATH: &str = "src/conf";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub logger: Logger,
}

impl Config {
    pub fn init() -> Self {
        let env = env::var("environment").unwrap_or_else(|_| "development".to_string());
        Self::load_config(env).unwrap()
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
