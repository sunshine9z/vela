use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::persistence::init::get_db;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role_id: i64,
    pub identity_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn find_by_username(username: &str) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(db)
            .await
    }
}
