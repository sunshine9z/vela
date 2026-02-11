use async_trait::async_trait;

use crate::commons::error::JobDomainError;
use crate::entity::job::{CreateJobDto, UpdateJobDto};

/// Job 仓库接口
///
/// 定义了 Job 相关的数据库操作抽象
#[async_trait]
pub trait JobRepositoryTrait: Send + Sync {
    /// 创建定时任务
    ///
    /// # 参数
    /// - `job`: 创建任务的数据传输对象
    ///
    /// # 返回
    /// - 成功：返回创建的任务ID
    /// - 失败：返回领域错误
    async fn create(&self, job: CreateJobDto) -> Result<i64, JobDomainError>;

    /// 根据ID删除定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回领域错误
    async fn delete_by_id(&self, id: i64) -> Result<(), JobDomainError>;

    /// 根据ID更新定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    /// - `job`: 更新任务的数据传输对象
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回领域错误
    async fn update_by_id(&self, id: i64, job: UpdateJobDto) -> Result<(), JobDomainError>;
}
