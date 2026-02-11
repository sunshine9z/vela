use jobDomain::entity::job::{CreateJobDto, UpdateJobDto};
use queryx::{
    corn_job::entity::{JobVo, ListJobQo},
    entity::PageReq,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate, Default)]
pub struct CreateReq {
    pub name: String,
    pub class: String,
    pub cron: String,
    pub queue: String,
    pub args: String,
    pub retry: bool,
}

impl From<CreateReq> for CreateJobDto {
    fn from(value: CreateReq) -> Self {
        Self {
            name: value.name,
            class: value.class,
            cron: value.cron,
            queue: Some(value.queue),
            args: Some(value.args),
            retry: Some(value.retry),
            ..Default::default()
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Validate, Default)]
pub struct UpdateReq {
    pub id: i64,
    pub name: Option<String>,
    pub class: Option<String>,
    pub cron: Option<String>,
    pub queue: Option<String>,
    pub args: Option<String>,
    pub retry: Option<bool>,
}

impl From<UpdateReq> for UpdateJobDto {
    fn from(value: UpdateReq) -> Self {
        Self {
            id: value.id,
            name: value.name,
            class: value.class,
            cron: value.cron,
            queue: value.queue,
            args: value.args,
            retry: value.retry,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Validate, Default)]
pub struct ListReq {
    pub page_req: PageReq,
    pub class: Option<String>,
}

impl Into<ListJobQo> for ListReq {
    fn into(self) -> ListJobQo {
        ListJobQo {
            page_req: self.page_req,
            class: self.class,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRes {
    pub jobs: Vec<JobInfoRes>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JobInfoRes {
    pub id: i64,
    pub name: String,
    pub class: String,
    pub cron: String,
    pub queue: String,
    pub args: String,
    pub retry: bool,
}

impl From<JobVo> for JobInfoRes {
    fn from(value: JobVo) -> Self {
        Self {
            id: value.id as i64,
            name: value.name,
            class: value.class,
            cron: value.cron,
            queue: value.queue.unwrap_or_default(),
            args: value.args.unwrap_or_default(),
            retry: value.retry.unwrap_or_default(),
        }
    }
}

impl From<CreateJobDto> for JobInfoRes {
    fn from(value: CreateJobDto) -> Self {
        Self {
            id: value.id as i64,
            name: value.name,
            class: value.class,
            cron: value.cron,
            queue: value.queue.unwrap_or_default(),
            args: value.args.unwrap_or_default(),
            retry: value.retry.unwrap_or_default(),
        }
    }
}
