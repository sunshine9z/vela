use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::persistence::entities::users;
use crate::persistence::id_gen::next_id;
use crate::persistence::init::get_db;

impl users::Model {
    pub async fn find_by_username(username: &str) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
    }

    pub async fn find_by_id(id: i64) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        users::Entity::find_by_id(id).one(db).await
    }

    pub async fn create(user: user_domain::entity::user::User) -> Result<i64, DbErr> {
        let db = get_db().await;
        let id = next_id();
        let u = users::ActiveModel {
            id: Set(id),
            name: Set(user.name.unwrap_or_default()),
            username: Set(user.username),
            password: Set(user.password),
            role_id: Set(Option::Some(user.role_id)),
            identity_code: Set(user.identity_code),
            phone: Set(user.phone),
            email: Set(user.email),
            sex: Set(user.sex),
            avatar: Set(user.avatar),
            status: Set(user.status),
            remark: Set(user.remark),
            created_at: Set(Option::Some(Local::now().naive_local())),
            updated_at: Set(Option::Some(Local::now().naive_local())),
            create_by: Set(user.create_by.unwrap_or_default()),
            update_by: Set(user.update_by.unwrap_or_default()),

            ..Default::default()
        };
        let ret = users::Entity::insert(u).exec(db).await?;
        Ok(ret.last_insert_id as i64)
    }

    pub async fn update_by_id(id: i64, user: user_domain::entity::user::User) -> Result<(), DbErr> {
        let db = get_db().await;
        let u = users::ActiveModel {
            name: Set(user.name.unwrap_or_default()),
            username: Set(user.username),
            password: Set(user.password),
            role_id: Set(Option::Some(user.role_id)),
            identity_code: Set(user.identity_code),
            phone: Set(user.phone),
            email: Set(user.email),
            sex: Set(user.sex),
            avatar: Set(user.avatar),
            status: Set(user.status),
            remark: Set(user.remark),
            updated_at: Set(Option::Some(Local::now().naive_local())),
            update_by: Set(user.update_by.unwrap_or_default()),
            ..Default::default()
        };
        let _ = users::Entity::update(u)
            .filter(users::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_by_id(id: i64) -> Result<(), DbErr> {
        let db = get_db().await;
        users::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
