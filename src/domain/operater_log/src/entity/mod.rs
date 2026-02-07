use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct OperaterLog {
    pub id: i64,
    pub api_name: String,
    pub oper_ip: String,
    pub oper_id: i64,
    pub oper_name: String,
    pub oper_url: String,
    pub oper_location: String,
    pub request_method: String,
    pub oper_param: String,
    pub json_result: String,
    pub cost_time: i64,
    pub oper_time: DateTime<Local>,
}
