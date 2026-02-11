use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 插入 admin 用户
        manager.get_connection()
            .execute(
                Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"
                    INSERT INTO users (id, name, username, password, role_id, phone, email, status, created_at, updated_at, create_by, update_by)
                    VALUES (1,'管理员','admin','$argon2id$v=19$m=19456,t=2,p=1$WMN7u3nv4YgA5IZMubXYtg$lrdE3M+feoyzfXrqD4iMlC8rhOBdRi93+v5pFYJh7Qo',0,'','',1,now(),now(),0,0)
                    "#,
                    vec![]
                ),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除 admin 用户
        manager
            .get_connection()
            .execute(Statement::from_sql_and_values(
                manager.get_database_backend(),
                r#"DELETE FROM users WHERE username = ?"#,
                vec!["admin".into()],
            ))
            .await?;

        Ok(())
    }
}
