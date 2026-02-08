pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260206_020910_sys_job;
mod m20260207_124414_init;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260206_020910_sys_job::Migration),
            Box::new(m20260207_124414_init::Migration),
        ]
    }
}
