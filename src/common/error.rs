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
}
