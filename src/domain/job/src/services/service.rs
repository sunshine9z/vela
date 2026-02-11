use async_trait::async_trait;

use crate::MODEL_JOB_DOMAIN;
use crate::entity::job::UpdateJobDto;
use crate::{
    api::traits::JobDomainTrait, commons::error::JobDomainError, entity::job::CreateJobDto,
};
use tracing::info;

/// Job 领域服务实现
///
/// 实现了 JobDomainTrait 接口，处理 Job 相关的业务逻辑
pub struct JobDomainImpl {
    /// Job 仓库，处理数据库操作
    pub job_repo: Box<dyn crate::repository::job::JobRepositoryTrait + Sync + Send>,
}

#[async_trait]
impl JobDomainTrait for JobDomainImpl {
    /// 创建定时任务
    ///
    /// # 参数
    /// - `job`: 创建任务的数据传输对象
    ///
    /// # 返回
    /// - 成功：返回创建的任务ID
    /// - 失败：返回领域错误
    async fn create(&self, job: CreateJobDto) -> Result<i64, JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Creating job: {}", job.name);
        self.job_repo.create(job).await
    }

    /// 根据ID删除定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回领域错误
    async fn delete_by_id(&self, id: i64) -> Result<(), JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Deleting job with id: {}", id);
        self.job_repo.delete_by_id(id).await
    }

    /// 根据ID更新定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    /// - `job`: 更新任务的数据传输对象
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回领域错误
    async fn update_by_id(&self, id: i64, job: UpdateJobDto) -> Result<(), JobDomainError> {
        info!(target: MODEL_JOB_DOMAIN, "Updating job with id: {}", id);
        self.job_repo.update_by_id(id, job).await
    }
}
