use std::error::Error;

use tracing::info;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, prelude::*};
use tracing_subscriber::{
    Layer, Registry, filter,
    fmt::{self, time::FormatTime},
};

use crate::{
    config::{APP_CONFIG, config::LogLevel},
    logger::{API_LOG, MODULE_NAME},
};

#[derive(Debug, Clone)]
pub struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f");
        write!(w, "{}", formatted)
    }
}

pub fn init() -> Result<Vec<WorkerGuard>, Box<dyn Error>> {
    let mut guards = Vec::new();

    // 消费log门面日志 转为 tracing Event日志
    LogTracer::builder()
        // .with_max_level(log::LevelFilter::Error)
        .init()
        .expect(format!("{MODULE_NAME} LogTracer 初始化失败").as_str());

    let log_conf = APP_CONFIG.logger.clone();

    let format = fmt::format()
        .with_level(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_timer(LocalTimer);

    // 1. 控制台
    println!("{MODULE_NAME}: 加载控制台日志...");
    let (console_non_blocking, console_guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(console_guard);

    // 2. web日志文件
    println!("{MODULE_NAME}: 加载web日志...");
    let web_file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_conf.log_dir.clone(),
        log_conf.web_file_name,
    );
    let (web_file_appender, web_guard) = tracing_appender::non_blocking(web_file_appender);
    guards.push(web_guard);

    // 3. api日志文件
    println!("{MODULE_NAME}: 加载api日志...");
    let api_file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_conf.log_dir.clone(),
        log_conf.api_file_name,
    );
    let (api_file_appender, api_guard) = tracing_appender::non_blocking(api_file_appender);
    guards.push(api_guard);

    let get_level = match log_conf.level {
        LogLevel::Trace => tracing::Level::TRACE,
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Error => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    // 集合
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(get_level.into()))
        .with(
            fmt::Layer::default()
                .with_ansi(true)
                .with_target(true)
                .with_writer(console_non_blocking)
                .event_format(format.clone().pretty()),
        )
        .with(
            fmt::Layer::default()
                .with_ansi(false)
                .with_target(true)
                .with_writer(web_file_appender)
                .event_format(format.clone().compact())
                .with_filter(filter::filter_fn(|metadata| metadata.target() != API_LOG)),
        )
        .with(
            fmt::Layer::default()
                .with_ansi(false)
                .with_target(true)
                .with_writer(api_file_appender)
                .event_format(format.clone().compact())
                .with_filter(filter::filter_fn(|metadata| metadata.target() == API_LOG)),
        );
    tracing::subscriber::set_global_default(subscriber)?;
    info!("{MODULE_NAME}: 日志初始化完成");
    Ok(guards)
}
