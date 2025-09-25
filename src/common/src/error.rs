use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    // web错误
    #[error("请求错误(404): {0}")]
    BadRequest(String),
    // 未授权
    #[error("未授权(401), 错误信息: {0}")]
    E401(String),
    // 未找到
    #[error("资源不存在(404), 错误信息: {0}")]
    E404(String),
    // 自定义状态码错误
    #[error("状态码: {0}, 错误信息: {1}")]
    WithStatus(StatusCode, String),
    // 内部错误-配置错误
    #[error("内部错误(500), 配置错误: {0}")]
    ConfigError(String),
    // 内部错误-日志错误
    #[error("内部错误(500), 日志配置错误: {0}")]
    LoggerError(String),
    // 内部错误-缓存错误
    #[error("内部错误(500), 初始化缓存错误: {0}")]
    CacheInitError(String),
    #[error("内部错误(500), redis错误: {0}")]
    RedisError(String),
    // 内部错误-数据库错误
    #[error("内部错误(500), 错误信息: {0}")]
    DBError(String),
    #[error("内部错误(500), 错误信息: {0}")]
    InternalError(String),
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::InternalError(msg.into())
    }
}
impl From<String> for AppError {
    fn from(arg: String) -> Self {
        AppError::InternalError(arg)
    }
}
impl From<()> for AppError {
    fn from(_: ()) -> Self {
        AppError::InternalError("no found".to_owned())
    }
}
impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(arg: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::InternalError(arg.to_string())
    }
}
