use sea_orm_migration::prelude::*;
use migration::{Migrator, MigratorTrait};

#[async_std::main]
async fn main() {
  let database_url = "sqlite://db-path-here";
  let connection = sea_orm::Database::connect(database_url).await.unwrap();
  Migrator::up(&connection, None).await.unwrap();
}