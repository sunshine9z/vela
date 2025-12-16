use async_trait::async_trait;
use commonx::error::AppError;
use serde_json::Value as JsonValue;

#[async_trait]
pub trait Worker: Send + Sync {
    fn disable_argument_coercion(&self) -> bool {
        false
    }

    #[must_use]
    fn class_name() -> String
    where
        Self: Sized,
    {
        use convert_case::{Case, Casing};
        let type_name = std::any::type_name::<Self>();
        let name = type_name.split("::").last().unwrap_or(type_name);
        name.to_case(Case::UpperCamel)
    }

    fn max_retries(&self) -> usize {
        1
    }

    async fn perform(&self, args: JsonValue) -> Result<(), AppError>;
}

#[async_trait]
pub trait AppWorker: Worker {
    fn new() -> Self;
    //同步加入队列
    async fn enqueue_sync(args: JsonValue) -> Result<(), AppError> {
        todo!()
    }
    // 异步加入队列
    async fn enqueue_async(args: JsonValue) -> Result<(), AppError> {
        todo!()
    }
    // 异步执行
    async fn execute_async(args: JsonValue) -> Result<(), AppError> {
        todo!()
    }
    // 同步执行
    async fn execute_sync(args: JsonValue) -> Result<(), AppError> {
        todo!()
    }
}

pub struct WorkerRef {
    worker: Box<dyn Worker>,
    max_retries: usize,
}

impl WorkerRef {
    pub(crate) fn new<W>(worker: W) -> WorkerRef
    where
        W: Worker + 'static,
    {
        let max_retries = worker.max_retries();
        WorkerRef {
            worker: Box::new(worker),
            max_retries,
        }
    }
}
