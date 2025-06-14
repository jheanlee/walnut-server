use axum::body::Body;
use axum::{http, Json};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use serde_json::json;
use entity::entities::password;
use crate::common::error::ApiError;
use crate::orm::password::{db_delete_password, db_get_password, db_list_passwords, db_modify_password, db_new_password, PasswordItem};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Items {
  passwords: Vec<password::PartialModel>,
}
pub async fn list_items() -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let items = Items {
    passwords: db_list_passwords().await?
  };
  let response_body = Body::from(serde_json::to_string(&items)?);
  let response = response_builder.body(response_body)?;
  Ok(response)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PasswordCreation {
  pub master: String,
  pub website: String,
  pub username: Option<String>,
  pub email: Option<String>,
  pub encrypted_password: String,
  pub notes: Option<String>
}
pub async fn new_password_item(Json(password_creation): Json<PasswordCreation>) -> Result<impl IntoResponse, ApiError> {
  db_new_password(PasswordItem {
    master: password_creation.master,
    website: password_creation.website,
    username: password_creation.username,
    email: password_creation.email,
    encrypted_password: password_creation.encrypted_password,
    notes: password_creation.notes,
  }).await?;
  
  Ok(StatusCode::OK)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PasswordModification {
  pub id: i32,
  pub master: String,
  pub website: String,
  pub username: Option<String>,
  pub email: Option<String>,
  pub encrypted_password: String,
  pub notes: Option<String>
}
pub async fn modify_password_item(Json(password_modification): Json<PasswordModification>) -> Result<impl IntoResponse, ApiError> {
  let res = db_modify_password(password_modification.id, PasswordItem {
    master: password_modification.master,
    website: password_modification.website,
    username: password_modification.username,
    email: password_modification.email,
    encrypted_password: password_modification.encrypted_password,
    notes: password_modification.notes,
  }).await?;
  
  if res {
    Ok(StatusCode::OK)
  } else {
    Ok(StatusCode::NOT_FOUND)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PasswordDeletion {
  pub id: i32,
}
pub async fn delete_password_item(Json(password_deletion) : Json<PasswordDeletion>) -> Result<impl IntoResponse, ApiError> {
  let res = db_delete_password(password_deletion.id).await?;
  if res == 0 {
    Ok(StatusCode::NOT_FOUND)
  } else {
    Ok(StatusCode::OK)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PasswordQuery {
  pub id: i32,
}
pub async fn get_password_item(Json(password_query): Json<PasswordQuery>) -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let res = db_get_password(password_query.id).await?;
  if let Some(encrypted_password) = res {
    let response_body = Body::from(json!({
      "encrypted_password": encrypted_password
    }).to_string());
    Ok(response_builder.body(response_body)?)
  } else {
    Ok(response_builder.status(StatusCode::NOT_FOUND).body(Body::from(""))?)
  }
}