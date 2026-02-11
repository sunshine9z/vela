//! Corn Job Controller
//! 
//! 定时任务控制器，处理定时任务相关的HTTP请求

use axum::response::IntoResponse;
use commonx::error::AppError;
use jobDomain::JobDomainTrait;
use queryx::corn_job::api::JobQueryTrait;

use crate::common::validated_json::VJson;
use crate::controller::CORN_JOB_CONTROLLER;
use crate::resp::ApiResponse;
use crate::types::corn_job::{CreateReq, JobInfoRes, ListReq, ListRes, UpdateReq};
use crate::types::GetByIdReq;

/// 创建定时任务
/// 
/// # 参数
/// - `arg`: 创建定时任务的请求参数
/// 
/// # 返回
/// - 成功：返回创建的任务ID
/// - 失败：返回错误信息
#[must_use]
pub async fn create(VJson(arg): VJson<CreateReq>) -> impl IntoResponse {
    ApiResponse::from_result(CORN_JOB_CONTROLLER.create(arg).await)
}

/// 更新定时任务
/// 
/// # 参数
/// - `arg`: 更新定时任务的请求参数
/// 
/// # 返回
/// - 成功：返回空
/// - 失败：返回错误信息
#[must_use]
pub async fn update_by_id(VJson(arg): VJson<UpdateReq>) -> impl IntoResponse {
    ApiResponse::from_result(CORN_JOB_CONTROLLER.update_by_id(arg).await)
}

/// 删除定时任务
/// 
/// # 参数
/// - `arg`: 删除定时任务的请求参数，包含任务ID
/// 
/// # 返回
/// - 成功：返回空
/// - 失败：返回错误信息
#[must_use]
pub async fn delete_by_id(VJson(arg): VJson<GetByIdReq>) -> impl IntoResponse {
    ApiResponse::from_result(CORN_JOB_CONTROLLER.delete_by_id(arg).await)
}

/// 获取定时任务详情
/// 
/// # 参数
/// - `arg`: 获取定时任务的请求参数，包含任务ID
/// 
/// # 返回
/// - 成功：返回任务详情
/// - 失败：返回错误信息
#[must_use]
pub async fn get_by_id(VJson(arg): VJson<GetByIdReq>) -> impl IntoResponse {
    ApiResponse::from_result(CORN_JOB_CONTROLLER.get_by_id(arg).await)
}

/// 获取定时任务列表
/// 
/// # 参数
/// - `arg`: 获取定时任务列表的请求参数，包含分页等信息
/// 
/// # 返回
/// - 成功：返回任务列表
/// - 失败：返回错误信息
#[must_use]
pub async fn list(VJson(arg): VJson<ListReq>) -> impl IntoResponse {
    ApiResponse::from_result(CORN_JOB_CONTROLLER.list(arg).await)
}

/// 定时任务控制器接口
/// 
/// 定义了定时任务控制器需要实现的方法
pub trait CornJobControllerTrait: Send + Sync {
    /// 创建定时任务
    /// 
    /// # 参数
    /// - `job`: 创建定时任务的请求数据
    /// 
    /// # 返回
    /// - 成功：返回创建的任务ID
    /// - 失败：返回错误信息
    async fn create(&self, job: CreateReq) -> Result<i64, AppError>;

    /// 更新定时任务
    /// 
    /// # 参数
    /// - `update_job`: 更新定时任务的请求数据
    /// 
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回错误信息
    async fn update_by_id(&self, update_job: UpdateReq) -> Result<(), AppError>;

    /// 删除定时任务
    /// 
    /// # 参数
    /// - `id`: 删除定时任务的请求数据，包含任务ID
    /// 
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回错误信息
    async fn delete_by_id(&self, id: GetByIdReq) -> Result<(), AppError>;

    /// 获取定时任务详情
    /// 
    /// # 参数
    /// - `id`: 获取定时任务的请求数据，包含任务ID
    /// 
    /// # 返回
    /// - 成功：返回任务详情
    /// - 失败：返回错误信息
    async fn get_by_id(&self, id: GetByIdReq) -> Result<Option<JobInfoRes>, AppError>;

    /// 获取定时任务列表
    /// 
    /// # 参数
    /// - `req`: 获取定时任务列表的请求数据，包含分页等信息
    /// 
    /// # 返回
    /// - 成功：返回任务列表
    /// - 失败：返回错误信息
    async fn list(&self, req: ListReq) -> Result<ListRes, AppError>;
}

/// 定时任务控制器实现
/// 
/// 实现了`CornJobControllerTrait`接口，处理定时任务的业务逻辑
pub struct CornJobController<J: JobDomainTrait + Send + Sync, Q: JobQueryTrait + Sync + Send> {
    /// 任务领域服务，处理任务的业务逻辑
    job_domain: J,
    /// 任务查询服务，处理任务的查询逻辑
    job_query: Q,
}

impl<J: JobDomainTrait + Send + Sync, Q: JobQueryTrait + Sync + Send> CornJobControllerTrait
    for CornJobController<J, Q>
{
    /// 创建定时任务
    async fn create(&self, job: CreateReq) -> Result<i64, AppError> {
        self.job_domain
            .create(job.into())
            .await
            .map_err(AppError::from)
    }

    /// 更新定时任务
    async fn update_by_id(&self, req: UpdateReq) -> Result<(), AppError> {
        self.job_domain
            .update_by_id(req.id, req.into())
            .await
            .map_err(AppError::from)
    }

    /// 删除定时任务
    async fn delete_by_id(&self, req: GetByIdReq) -> Result<(), AppError> {
        self.job_domain
            .delete_by_id(req.id)
            .await
            .map_err(AppError::from)
    }

    /// 获取定时任务详情
    async fn get_by_id(&self, id: GetByIdReq) -> Result<Option<JobInfoRes>, AppError> {
        self.job_query
            .get_by_id(id.id)
            .await
            .map(|job| job.map(JobInfoRes::from))
            .map_err(AppError::from)
    }

    /// 获取定时任务列表
    async fn list(&self, req: ListReq) -> Result<ListRes, AppError> {
        self.job_query
            .list(req.into())
            .await
            .map(|jobs| ListRes {
                jobs: jobs.into_iter().map(JobInfoRes::from).collect(),
            })
            .map_err(AppError::from)
    }
}

impl<J: JobDomainTrait + Send + Sync, Q: JobQueryTrait + Sync + Send> CornJobController<J, Q> {
    /// 创建定时任务控制器实例
    /// 
    /// # 参数
    /// - `job_domain`: 任务领域服务
    /// - `job_query`: 任务查询服务
    /// 
    /// # 返回
    /// 定时任务控制器实例
    #[must_use]
    pub fn new(job_domain: J, job_query: Q) -> Self {
        Self {
            job_domain,
            job_query,
        }
    }
}
