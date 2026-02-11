use chrono::{DateTime, Local};

use crate::entity::PageReq;

#[derive(Clone, Debug)]
pub struct ListJobQo {
    pub page_req: PageReq,
    pub class: Option<String>,
}

impl Default for ListJobQo {
    fn default() -> Self {
        Self {
            page_req: PageReq::default(),
            class: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct JobVo {
    pub id: i64,
    pub name: String,
    pub class: String,
    pub cron: String,
    pub queue: Option<String>,
    pub args: Option<String>,
    pub retry: Option<bool>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
