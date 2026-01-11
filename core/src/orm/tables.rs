use sea_orm::{ConnectionTrait, Schema, Statement};
use entity::entities::{master, password};
use crate::common::error::ApiError;
use crate::SHARED_CELL;

pub async fn init_tables() -> Result<(), ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let schema = Schema::new(db.get_database_backend());
  let mut master_stmt = schema.create_table_from_entity(master::Entity);
  master_stmt.if_not_exists();
  db.execute(db.get_database_backend().build(&master_stmt)).await?;

  let mut password_stmt = schema.create_table_from_entity(password::Entity);
  password_stmt.if_not_exists();
  db.execute(db.get_database_backend().build(&password_stmt)).await?;
  Ok(())
}