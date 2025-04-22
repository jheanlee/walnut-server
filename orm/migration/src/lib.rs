pub use sea_orm_migration::prelude::*;

mod m20250422_000001_create_master_table;
mod m20250422_000002_create_password_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20250422_000001_create_master_table::Migration),
      Box::new(m20250422_000002_create_password_table::Migration)
    ]
  }
}
