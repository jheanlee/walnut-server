use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, Set};
use crate::common::error::ApiError;
use entity::entities::password;
use entity::entities::password::ActiveModel;
use entity::entities::prelude::Password;
use crate::SHARED_CELL;

pub async fn db_list_passwords() -> Result<Vec<password::PartialModel>, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  Ok(password::Entity::find().into_partial_model().all(db).await?)
}

pub struct PasswordItem {
  pub master: String,
  pub website: String,
  pub username: Option<String>,
  pub email: Option<String>,
  pub encrypted_password: String,
  pub notes: Option<String>
}

pub async fn db_new_password(item: PasswordItem) -> Result<(), ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();

  let password_model = ActiveModel {
    id: Default::default(),
    master: Set(item.master),
    website: Set(item.website),
    username: Set(item.username),
    email: Set(item.email),
    encrypted_password: Set(item.encrypted_password),
    notes: Set(item.notes),
  };

  Password::insert(password_model).exec(db).await?;
  Ok(())
}

pub async fn db_modify_password(id: i32, item: PasswordItem) -> Result<bool, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();

  let password_item = Password::find_by_id(id).one(db).await?;
  if let Some(password_item) = password_item {
    let mut password_model = password_item.into_active_model();
    password_model.master = Set(item.master);
    password_model.website = Set(item.website);
    password_model.username = Set(item.username);
    password_model.email = Set(item.email);
    password_model.encrypted_password = Set(item.encrypted_password);
    password_model.notes = Set(item.notes);
    password_model.update(db).await?;
    Ok(true)
  } else {
    Ok(false)
  }
}

pub async fn db_delete_password(id: i32) -> Result<u64, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let model = password::Entity::find_by_id(id).one(db).await?;
  if let Some(model) = model {
    let res = model.delete(db).await?;
    Ok(res.rows_affected)
  } else {
    Ok(0)
  }
}

pub async fn db_get_password(id: i32) -> Result<Option<String>, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let model = password::Entity::find_by_id(id).one(db).await?;
  if let Some(model) = model {
    Ok(Some(model.encrypted_password))
  } else {
    Ok(None)
  }
}
