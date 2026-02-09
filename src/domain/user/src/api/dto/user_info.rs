use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoDto {
    pub id: i64,
    pub role_id: i64,
    pub username: String,
    pub name: Option<String>,
    pub identity_code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub create_by: Option<i64>,
    pub created_at: Option<DateTime<Local>>,
    pub update_by: Option<i64>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<entity::user::User> for UserInfoDto {
    fn from(user: entity::user::User) -> Self {
        Self {
            id: user.id,
            role_id: user.role_id,
            username: user.username,
            name: user.name,
            identity_code: user.identity_code,
            email: user.email,
            phone: user.phone,
            sex: user.sex,
            avatar: user.avatar,
            status: user.status,
            remark: user.remark,
            create_by: user.create_by,
            created_at: user.created_at,
            update_by: user.update_by,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}
