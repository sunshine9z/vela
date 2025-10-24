use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub role_id: i64,
    pub username: String,
    pub name: Option<String>,
    pub identity_code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub password: String,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub create_by: Option<i64>,
    pub created_at: Option<DateTime<Local>>,
    pub update_by: Option<i64>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}
