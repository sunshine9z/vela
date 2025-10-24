use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::persistence::entities::users;
use crate::persistence::init::get_db;

impl users::Model {
    pub async fn find_by_username(username: &str) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
    }
}
