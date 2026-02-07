use crate::persistence::entities::sys_oper_log;
use crate::persistence::id_gen::next_id;
use crate::persistence::init::get_db;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{DbErr, EntityTrait};
impl sys_oper_log::Model {
    pub async fn create(
        api_name: String,
        method: String,
        request_method: String,
        oper_id: i64,
        oper_name: String,
        oper_url: String,
        oper_ip: String,
        oper_location: String,
        oper_param: String,
        json_result: String,
        cost_time: i64,
    ) -> Result<i64, DbErr> {
        let db = get_db().await;
        let id = next_id();
        let log = sys_oper_log::ActiveModel {
            id: Set(id),
            api_name: Set(api_name),
            method: Set(method),
            request_method: Set(request_method),
            oper_id: Set(Option::Some(oper_id)),
            oper_name: Set(oper_name),
            oper_url: Set(oper_url),
            oper_ip: Set(oper_ip),
            oper_location: Set(oper_location),
            oper_param: Set(oper_param),
            json_result: Set(json_result),
            oper_time: Set(Local::now().naive_local()),
            cost_time: Set(cost_time),
            ..Default::default()
        };
        let _ = sys_oper_log::Entity::insert(log).exec(db).await?;
        Ok(id)
    }

    pub async fn delete_by_id(id: i64) -> Result<(), DbErr> {
        let db = get_db().await;
        sys_oper_log::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn list() -> Result<Vec<Self>, DbErr> {
        let db = get_db().await;
        sys_oper_log::Entity::find().all(db).await
    }
}
