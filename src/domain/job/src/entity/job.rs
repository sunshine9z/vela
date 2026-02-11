use chrono::{DateTime, Local};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CreateJobDto {
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UpdateJobDto {
    pub id: i64,
    pub name: Option<String>,
    pub class: Option<String>,
    pub cron: Option<String>,
    pub queue: Option<String>,
    pub args: Option<String>,
    pub retry: Option<bool>,
}
