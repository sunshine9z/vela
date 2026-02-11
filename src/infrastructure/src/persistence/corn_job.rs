use crate::persistence::entities::corn_job;
use crate::persistence::id_gen::next_id;
use crate::persistence::init::get_db;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

impl corn_job::Model {
    /// 创建定时任务
    ///
    /// # 参数
    /// - `name`: 任务名称
    /// - `class`: 任务执行类
    /// - `cron`: Cron表达式
    /// - `queue`: 任务队列（可选）
    /// - `args`: 任务参数（可选）
    /// - `retry`: 是否重试（可选）
    ///
    /// # 返回
    /// - 成功：返回创建的任务ID
    /// - 失败：返回数据库错误
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

        corn_job::Entity::insert(job).exec(db).await?;
        Ok(id)
    }

    /// 根据ID删除定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回数据库错误
    pub async fn delete_by_id(id: i64) -> Result<(), DbErr> {
        let db = get_db().await;
        corn_job::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    /// 根据ID更新定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    /// - `name`: 任务名称（可选）
    /// - `class`: 任务执行类（可选）
    /// - `cron`: Cron表达式（可选）
    /// - `queue`: 任务队列（可选）
    /// - `args`: 任务参数（可选）
    /// - `retry`: 是否重试（可选）
    ///
    /// # 返回
    /// - 成功：返回空
    /// - 失败：返回数据库错误
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

    /// 根据ID查询定时任务
    ///
    /// # 参数
    /// - `id`: 任务ID
    ///
    /// # 返回
    /// - 成功：返回任务详情（可选）
    /// - 失败：返回数据库错误
    pub async fn find_by_id(id: i64) -> Result<Option<Self>, DbErr> {
        let db = get_db().await;
        corn_job::Entity::find_by_id(id).one(db).await
    }

    /// 查询定时任务列表（支持分页）
    ///
    /// # 参数
    /// - `page`: 页码（从1开始）
    /// - `page_size`: 每页大小
    ///
    /// # 返回
    /// - 成功：返回任务列表
    /// - 失败：返回数据库错误
    pub async fn list(page: Option<u64>, page_size: Option<u64>) -> Result<Vec<Self>, DbErr> {
        let db = get_db().await;
        let mut query = corn_job::Entity::find();

        // 实现真正的分页
        if let (Some(page), Some(page_size)) = (page, page_size) {
            let offset = (page - 1) * page_size;
            query = query.offset(offset).limit(page_size);
        }

        query.all(db).await
    }

    // /// 查询定时任务总数
    // ///
    // /// # 返回
    // /// - 成功：返回任务总数
    // /// - 失败：返回数据库错误
    // pub async fn count() -> Result<i64, DbErr> {
    //     let db = get_db().await;
    //     corn_job::Entity::find().count(db).await
    // }
}
