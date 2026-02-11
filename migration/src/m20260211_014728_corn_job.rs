use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create CornJob table
        manager
            .create_table(
                Table::create()
                    .table(CornJob::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CornJob::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CornJob::Name).string().not_null())
                    .col(ColumnDef::new(CornJob::Class).string().not_null())
                    .col(ColumnDef::new(CornJob::Cron).string().not_null())
                    .col(ColumnDef::new(CornJob::Queue).string().null())
                    .col(ColumnDef::new(CornJob::Args).string().null())
                    .col(ColumnDef::new(CornJob::Retry).boolean().default(false))
                    .col(
                        ColumnDef::new(CornJob::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CornJob::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop CornJob table
        manager
            .drop_table(Table::drop().table(CornJob::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CornJob {
    Table,
    Id,
    Name,
    Class,
    Cron,
    Queue,
    Args,
    Retry,
    CreatedAt,
    UpdatedAt,
}
