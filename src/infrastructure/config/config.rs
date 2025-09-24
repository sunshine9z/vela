use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::{env, fs};

use crate::common::error::AppError;

static CONFIG_PATH: &str = "src/conf";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
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
