use crate::common::error::ApiError;
use crate::SHARED_CELL;
use entity::entities::master;
use entity::entities::prelude::Master;
use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::sha::Sha256;
use sea_orm::{sea_query, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};

pub async fn db_list_master() -> Result<Vec<master::PartialModel>, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  Ok(master::Entity::find().into_partial_model().all(db).await?)
}

pub async fn db_is_present(username: String) -> Result<bool, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  Ok(master::Entity::find().filter(master::Column::Username.eq(username)).one(db).await?.is_some())
}

pub async fn db_modify_master(username: String, password: String) -> Result<(), ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();

  let mut salt = [0u8; 16];
  rand_bytes(&mut salt)?;
  let base64_salt = base64::encode_block(&salt);

  let mut hasher = Sha256::new();
  hasher.update(&salt);
  hasher.update(password.as_bytes());
  let base64_hashed_password = base64::encode_block(hasher.finish().as_ref());

  let master_model = master::ActiveModel {
    id: Default::default(),
    username: Set(username),
    hashed_password: Set(base64_hashed_password),
    master_salt: Set(base64_salt),
  };

  Master::insert(master_model).on_conflict(
    sea_query::OnConflict::column(master::Column::Username)
      .update_column(master::Column::HashedPassword)
      .update_column(master::Column::MasterSalt)
      .to_owned()
  ).exec(db).await?;

  Ok(())
}

pub async fn db_delete_master(username: String) -> Result<u64, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let model = master::Entity::find().filter(master::Column::Username.eq(username)).one(db).await?;
  if let Some(model) = model {
    let res = model.delete(db).await?;
    Ok(res.rows_affected)
  } else {
    Ok(0)
  }
}