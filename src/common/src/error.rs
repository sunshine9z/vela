use hyper::StatusCode;
use thiserror::Error;

use crate::traits::IntoStatusTuple;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppError {
    // web错误
    #[error("请求错误(404): {0}")]
    BadRequest(String),
    // 未授权
    #[error("未授权(401), 错误信息: {0}")]
    AuthError(String),
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

    #[error("内部错误(500), {0}")]
    CacheNotFoundError(String),
    // 内部错误-数据库错误
    #[error("内部错误(500), 错误信息: {0}")]
    DBError(String),
    #[error("内部错误(500), 错误信息: {0}")]
    InternalError(String),

    #[error("内部错误(400), 校验错误: {0}")]
    ValidationError(String),

    #[error("自定义错误({0}), 错误信息: {1}")]
    CustomError(StatusCode, String),

    #[error("内部错误(500), 未实现: {0}")]
    NotImplementedError(String),

    #[error("{inner}\n{backtrace}")]
    WithBacktrace {
        inner: Box<Self>,
        backtrace: Box<std::backtrace::Backtrace>,
    },
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         self.into_status_tuple().into_response()
//     }
// }

impl AppError {
    pub fn bt(self) -> Self {
        let backtrace = std::backtrace::Backtrace::capture();
        match backtrace.status() {
            std::backtrace::BacktraceStatus::Disabled
            | std::backtrace::BacktraceStatus::Unsupported => self,
            _ => Self::WithBacktrace {
                inner: Box::new(self),
                backtrace: Box::new(backtrace),
            },
        }
    }
}

impl IntoStatusTuple for AppError {
    fn into_status_tuple(self) -> (StatusCode, String) {
        match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::E404(msg) => (StatusCode::NOT_FOUND, msg),
            Self::WithStatus(status, msg) => (status, msg),
            Self::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
    }
}

// 错误转换宏
macro_rules! impl_error_from {
    ($($error_type:ty),* $(,)?) => {
        $(
            impl From<$error_type> for AppError {
                fn from(err: $error_type) -> Self {
                     AppError::InternalError(err.to_string())
                }
            }
        )*
    };
}

// 标准错误类型转换
impl_error_from!(
    std::io::Error,
    serde_json::Error,
    bb8_redis::redis::RedisError,
    bb8::RunError<bb8_redis::redis::RedisError>,
    Box<dyn std::error::Error>,
    sea_orm::DbErr,
);

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::InternalError(msg.into())
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

impl Clone for AppError {
    fn clone(&self) -> Self {
        AppError::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}
