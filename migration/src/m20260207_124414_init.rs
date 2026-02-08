use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 生成密码哈希
        let password = "admin123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| DbErr::Custom(format!("密码哈希生成失败: {}", e)))?
            .to_string();
        println!("admin 密码哈希: {}", password_hash);
        // 插入 admin 用户
        manager.get_connection()
            .execute(
                Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"
                    INSERT INTO users (id, name, username, password, role_id, phone, email, status, created_at, updated_at, create_by, update_by)
                    VALUES (1,'管理员','admin','$argon2id$v=19$m=19456,t=2,p=1$lXyeUZL/EZ12cDIQ3QdhHw$1n5Zhbl6s7/mlKDYg+NIfMTJr8cyBomzAKNpr5ZOHcg',0,'','',1,now(),now(),0,0)
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
