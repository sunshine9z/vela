use chrono::{DateTime, Local};

/// 创建定时任务的数据传输对象
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CreateJobDto {
    /// 任务ID
    pub id: i64,
    /// 任务名称
    pub name: String,
    /// 任务执行类
    pub class: String,
    /// Cron表达式
    pub cron: String,
    /// 任务队列（可选）
    pub queue: Option<String>,
    /// 任务参数（可选）
    pub args: Option<String>,
    /// 是否重试（可选）
    pub retry: Option<bool>,
    /// 创建时间
    pub created_at: DateTime<Local>,
    /// 更新时间
    pub updated_at: DateTime<Local>,
}

/// 更新定时任务的数据传输对象
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UpdateJobDto {
    /// 任务ID
    pub id: i64,
    /// 任务名称（可选）
    pub name: Option<String>,
    /// 任务执行类（可选）
    pub class: Option<String>,
    /// Cron表达式（可选）
    pub cron: Option<String>,
    /// 任务队列（可选）
    pub queue: Option<String>,
    /// 任务参数（可选）
    pub args: Option<String>,
    /// 是否重试（可选）
    pub retry: Option<bool>,
}

/// 定时任务列表请求数据传输对象
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ListJobReq {
    /// 页码（从1开始）
    pub page: Option<i64>,
    /// 每页大小
    pub page_size: Option<i64>,
}

/// 定时任务列表响应数据传输对象
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ListJobRes {
    /// 任务列表
    pub jobs: Vec<CreateJobDto>,
    /// 总数
    pub total: i64,
}
