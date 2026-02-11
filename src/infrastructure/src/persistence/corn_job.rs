use crate::persistence::entities::corn_job;
use crate::persistence::id_gen::next_id;
use crate::persistence::init::get_db;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

impl corn_job::Model {
    pub async fn create(
        name: String,
        class: String,
        cron: String,
        queue: Option<String>,
        args: Option<String>,
        retry: Option<bool>,
    ) -> Result<i64, DbErr> {
        let db = get_db().await;
        let now = Local::now().naive_local();
        let id = next_id();
        let job = corn_job::ActiveModel {
            id: Set(id),
            name: Set(name),
            class: Set(class),
            cron: Set(cron),
            queue: Set(queue),
            args: Set(args),
            retry: Set(retry),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let _ = corn_job::Entity::insert(job).exec(db).await?;
        Ok(id)
    }

    pub async fn delete_by_id(id: i64) -> Result<(), DbErr> {
        let db = get_db().await;
        corn_job::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn update_by_id(
        id: i64,
        name: Option<String>,
        class: Option<String>,
        cron: Option<String>,
        queue: Option<String>,
        args: Option<String>,
        retry: Option<bool>,
    ) -> Result<(), DbErr> {
        let db = get_db().await;
        let now = Local::now().naive_local();
        let mut job = corn_job::ActiveModel {
            updated_at: Set(now),
            ..Default::default()
        };

        if let Some(name) = name {
            job.name = Set(name);
        }
        if let Some(class) = class {
            job.class = Set(class);
        }
        if let Some(cron) = cron {
            job.cron = Set(cron);
        }
        if let Some(queue) = queue {
            job.queue = Set(Some(queue));
        }
        if let Some(args) = args {
            job.args = Set(Some(args));
        }
        if let Some(retry) = retry {
            job.retry = Set(Some(retry));
        }

        corn_job::Entity::update(job)
            .filter(corn_job::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(id: i64) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        corn_job::Entity::find_by_id(id).one(db).await
    }

    pub async fn list(page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Self>, DbErr> {
        let db = get_db().await;
        corn_job::Entity::find().all(db).await
    }
}
