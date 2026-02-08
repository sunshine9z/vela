pub mod logger;

pub use logger::init;

pub static MODULE_NAME: &str = "[日志]";

pub const WEB_LOG: &'static str = "web";
pub const API_LOG: &'static str = "api";

/// Web信息日志宏，基于tracing::info!实现
/// 用法与tracing::info!相同，但专为Web应用场景优化
// 直接定义所有需要的日志宏，避免复杂的嵌套宏问题
#[macro_export]
macro_rules! web_info {
    ($($arg:tt)*) => {
        tracing::info!(target: "web", $($arg)*);
    };
}

/// Web跟踪日志宏，基于tracing::trace!实现
/// 用法与tracing::trace!相同，但专为Web应用场景优化
#[macro_export]
macro_rules! web_trace {
    ($($arg:tt)*) => {
        tracing::trace!(target: "web", $($arg)*);
    };
}

/// Web调试日志宏，基于tracing::debug!实现
/// 用法与tracing::debug!相同，但专为Web应用场景优化
#[macro_export]
macro_rules! web_debug {
    ($($arg:tt)*) => {
        tracing::debug!(target: "web", $($arg)*);
    };
}

/// Web错误日志宏，基于tracing::error!实现
/// 用法与tracing::error!相同，但专为Web应用场景优化
#[macro_export]
macro_rules! web_error {
    ($($arg:tt)*) => {
        tracing::error!(target: "web", $($arg)*);
    };
}

/// Web警告日志宏，基于tracing::warn!实现
/// 用法与tracing::warn!相同，但专为Web应用场景优化
#[macro_export]
macro_rules! web_warn {
    ($($arg:tt)*) => {
        tracing::warn!(target: "web", $($arg)*);
    };
}

/// API信息日志宏，基于tracing::info!实现
/// 用法与tracing::info!相同，但专为API应用场景优化
#[macro_export]
macro_rules! api_info {
    ($($arg:tt)*) => {
        tracing::info!(target: "api", $($arg)*);
    };
}

/// API跟踪日志宏，基于tracing::trace!实现
/// 用法与tracing::trace!相同，但专为API应用场景优化
#[macro_export]
macro_rules! api_trace {
    ($($arg:tt)*) => {
        tracing::trace!(target: "api", $($arg)*);
    };
}

/// API调试日志宏，基于tracing::debug!实现
/// 用法与tracing::debug!相同，但专为API应用场景优化
#[macro_export]
macro_rules! api_debug {
    ($($arg:tt)*) => {
        tracing::debug!(target: "api", $($arg)*);
    };
}

/// API错误日志宏，基于tracing::error!实现
/// 用法与tracing::error!相同，但专为API应用场景优化
#[macro_export]
macro_rules! api_error {
    ($($arg:tt)*) => {
        tracing::error!(target: "api", $($arg)*);
    };
}

/// API警告日志宏，基于tracing::warn!实现
/// 用法与tracing::warn!相同，但专为API应用场景优化
#[macro_export]
macro_rules! api_warn {
    ($($arg:tt)*) => {
        tracing::warn!(target: "api", $($arg)*);
    };
}
