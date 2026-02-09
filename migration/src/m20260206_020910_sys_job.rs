use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysOperLog::Table)
                    .if_not_exists()
                    .col(
                        big_integer(SysOperLog::Id)
                            .primary_key()
                            .comment("日志主键"),
                    )
                    .col(
                        big_integer(SysOperLog::OperId)
                            .default(0)
                            .comment("操作人员id"),
                    )
                    .col(
                        string(SysOperLog::ApiName)
                            .string_len(50)
                            .default("")
                            .comment("API名称"),
                    )
                    .col(
                        string(SysOperLog::RequestMethod)
                            .string_len(10)
                            .default("")
                            .comment("请求方式"),
                    )
                    .col(
                        string(SysOperLog::OperName)
                            .string_len(50)
                            .default("")
                            .comment("操作人员"),
                    )
                    .col(
                        string(SysOperLog::OperUrl)
                            .string_len(255)
                            .default("")
                            .comment("请求URL"),
                    )
                    .col(
                        string(SysOperLog::OperIp)
                            .string_len(128)
                            .default("")
                            .comment("主机地址"),
                    )
                    .col(
                        string(SysOperLog::OperLocation)
                            .string_len(255)
                            .default("")
                            .comment("操作地点"),
                    )
                    .col(
                        string(SysOperLog::OperParam)
                            .text()
                            .default("")
                            .comment("请求参数"),
                    )
                    .col(
                        string(SysOperLog::JsonResult)
                            .text()
                            .default("")
                            .comment("返回参数"),
                    )
                    .col(timestamp(SysOperLog::OperTime).comment("操作时间"))
                    .col(
                        big_integer(SysOperLog::CostTime)
                            .default(0)
                            .comment("消耗时间"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(SysOperLog::Table)
                    .if_not_exists()
                    .name("idx_sys_oper_log_oper_time")
                    .col(SysOperLog::OperTime)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysOperLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SysOperLog {
    Table,
    Id,
    ApiName,
    RequestMethod,
    OperId,
    OperName,
    OperUrl,
    OperIp,
    OperLocation,
    OperParam,
    JsonResult,
    OperTime,
    CostTime,
}
