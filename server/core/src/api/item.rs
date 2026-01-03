use axum::body::Body;
use axum::{http, Json};
use axum::extract::Path;
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::IntoResponse;
use serde_json::json;
use entity::entities::password;
use crate::auth::jwt::get_sub;
use crate::common::error::ApiError;
use crate::orm::password::{db_delete_password, db_get_password, db_list_passwords, db_modify_password, db_new_password, PasswordItem};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Items {
  passwords: Vec<password::PartialModel>,
}
pub async fn list_items(header_map: HeaderMap) -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let items = Items {
    passwords: db_list_passwords(
      get_sub(
        header_map.get("Authorization")
          .ok_or(ApiError::StatusCode(StatusCode::UNAUTHORIZED))?
          .to_str()?)
        .await.map_err(|e| ApiError::StatusCode(e))?
    ).await?
  };
  let response_body = Body::from(serde_json::to_string(&items)?);
  let response = response_builder.body(response_body)?;
  Ok(response)
}

//  TODO password sub verification

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewPasswordPath {
  pub user_id: String
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewPasswordBody {
  pub name: String,
  pub websites: String,
  pub username: String,
  pub email: String,
  pub encrypted_password: String,
  pub notes: String
}
pub async fn new_password_item(Path(path): Path<NewPasswordPath>, Json(body): Json<NewPasswordBody>) -> Result<impl IntoResponse, ApiError> {
  db_new_password(path.user_id, PasswordItem {
    name: body.name,
    website: body.websites,
    username: body.username,
    email: body.email,
    encrypted_password: body.encrypted_password,
    notes: body.notes,
  }).await?;
  
  Ok(StatusCode::OK)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdatePasswordPath {
  pub user_id: String,
  pub item_id: i32
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdatePasswordBody {
  pub name: String,
  pub websites: String,
  pub username: String,
  pub email: String,
  pub encrypted_password: String,
  pub notes: String
}
pub async fn update_password_item(Path(path): Path<UpdatePasswordPath>, Json(body): Json<UpdatePasswordBody>) -> Result<impl IntoResponse, ApiError> {
  db_modify_password(path.user_id, path.item_id, PasswordItem {
    name: body.name,
    website: body.websites,
    username: body.username,
    email: body.email,
    encrypted_password: body.encrypted_password,
    notes: body.notes,
  }).await?;
  
  Ok(StatusCode::OK)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeletePasswordPath {
  pub user_id: String,
  pub item_id: i32,
}
pub async fn delete_password_item(Path(path) : Path<DeletePasswordPath>) -> Result<impl IntoResponse, ApiError> {
  let res = db_delete_password(path.user_id, path.item_id).await?;
  if res == 0 {
    Ok(StatusCode::NOT_FOUND)
  } else {
    Ok(StatusCode::OK)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetPasswordPath {
  pub user_id: String,
  pub item_id: i32,
}
pub async fn get_password_item(Path(path): Path<GetPasswordPath>) -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let res = db_get_password(path.user_id, path.item_id).await?;
  let response_body = Body::from(serde_json::to_string(&res)?);
  Ok(response_builder.body(response_body)?)
}