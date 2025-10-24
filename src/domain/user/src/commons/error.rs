use commonx::error::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error("用户不存在:{0}")]
    UserNotFound(String),

    #[error("验证码不存在")]
    CaptchaNotFound,

    #[error("验证码错误:{0}")]
    CaptchaError(String),

    #[error("认证错误:{0}")]
    AuthError(String),

    #[error("内部错误:{0}")]
    InternalError(String),

    #[error("数据库错误:{0}")]
    DbError(String),

    #[error("内部错误(500), 未实现: {0}")]
    NotImplementedError(String),
}

impl From<UserDomainError> for AppError {
    fn from(e: UserDomainError) -> Self {
        match e {
            _ => AppError::InternalError(e.to_string()),
        }
    }
}
