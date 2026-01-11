use axum::http;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use crate::common::error::ApiError;
use entity::entities::password;
use entity::entities::password::{ActiveModel, ItemPartialModel};
use entity::entities::prelude::Password;
use crate::SHARED_CELL;

pub async fn db_list_passwords(master_id: String) -> Result<Vec<password::PartialModel>, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  Ok(password::Entity::find().filter(password::Column::Master.eq(master_id)).into_partial_model().all(db).await?)
}

pub struct PasswordItem {
  pub website: String,
  pub name: String,
  pub username: String,
  pub email: String,
  pub encrypted_password: String,
  pub notes: String
}

pub async fn db_new_password(user_id: String, item: PasswordItem) -> Result<(), ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();

  let password_model = ActiveModel {
    id: Default::default(),
    master: Set(user_id),
    name: Set(item.name),
    website: Set(item.website),
    username: Set(item.username),
    email: Set(item.email),
    encrypted_password: Set(item.encrypted_password),
    notes: Set(item.notes),
  };

  Password::insert(password_model).exec(db).await?;
  Ok(())
}

pub async fn db_modify_password(user_id: String, item_id: i32, item: PasswordItem) -> Result<(), ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();

  let password_item = Password::find_by_id(item_id).one(db).await?;
  if let Some(password_item) = password_item {
    if password_item.master == user_id {
      let mut password_model = password_item.into_active_model();
      password_model.name = Set(item.name);
      password_model.website = Set(item.website);
      password_model.username = Set(item.username);
      password_model.email = Set(item.email);
      password_model.encrypted_password = Set(item.encrypted_password);
      password_model.notes = Set(item.notes);
      password_model.update(db).await?;
      Ok(())
    } else {
      Err(ApiError::StatusCode(http::StatusCode::NOT_FOUND))
    }
  } else {
    Err(ApiError::StatusCode(http::StatusCode::NOT_FOUND))
  }
}

pub async fn db_delete_password(user_id: String, item_id: i32) -> Result<u64, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let model = password::Entity::find_by_id(item_id).one(db).await?;
  if let Some(model) = model {
    if model.master == user_id {
      let res = model.delete(db).await?;
      Ok(res.rows_affected)
    } else {
      Err(ApiError::StatusCode(http::StatusCode::NOT_FOUND))
    }
  } else {
    Err(ApiError::StatusCode(http::StatusCode::NOT_FOUND))
  }
}

pub async fn db_delete_password_by_master(master: String) -> Result<u64, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let res = password::Entity::delete_many()
    .filter(password::Column::Master.eq(master)).exec(db).await?;
  Ok(res.rows_affected)
}

pub async fn db_get_password(user_id: String, item_id: i32) -> Result<ItemPartialModel, ApiError> {
  let db = SHARED_CELL.get().unwrap().database_connection.as_ref().unwrap();
  let model: ItemPartialModel = password::Entity::find_by_id(item_id)
    .filter(password::Column::Master.eq(user_id)).into_partial_model().one(db).await?
    .ok_or(ApiError::StatusCode(http::StatusCode::NOT_FOUND))?;
  Ok(model)
}
