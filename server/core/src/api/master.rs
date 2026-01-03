use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::{http, Json};
use axum::response::IntoResponse;
use serde_json::json;
use crate::auth::jwt::generate_token;
use crate::common::error::ApiError;
use crate::orm::master::{db_authenticate_master, db_delete_master, db_is_present, db_list_master, db_modify_master};
use crate::orm::password::db_delete_password_by_master;

pub async fn list_master() -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let master_items = db_list_master().await?;
  let response_body = Body::from(serde_json::to_string(&master_items)?);
  let response = response_builder.body(response_body)?;
  Ok(response)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MasterModification {
  username: String,
  password: String,
  admin: bool
}
pub async fn new_master(Json(master_modification): Json<MasterModification>) -> Result<impl IntoResponse, ApiError> {
  let is_present = db_is_present(master_modification.username.clone()).await?;
  if !is_present {
    db_modify_master(master_modification.username, master_modification.password, master_modification.admin).await?;
    Ok(StatusCode::OK)
  } else {
    Ok(StatusCode::CONFLICT)
  }
}

pub async fn modify_master(Json(master_modification): Json<MasterModification>) -> Result<impl IntoResponse, ApiError> {
  let is_present = db_is_present(master_modification.username.clone()).await?;
  if is_present {
    db_modify_master(master_modification.username, master_modification.password, master_modification.admin).await?;
    Ok(StatusCode::OK)
  } else {
    Ok(StatusCode::NOT_FOUND)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MasterDeletion {
  username: String
}
pub async fn delete_master(Json(master_deletion): Json<MasterDeletion>) -> Result<impl IntoResponse, ApiError> {
  let res = db_delete_master(master_deletion.username.clone()).await?;
  let _password_delete_res = db_delete_password_by_master(master_deletion.username).await?;
  if res == 0 {
    Ok(StatusCode::NOT_FOUND)
  } else {
    Ok(StatusCode::OK)
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MasterLogin {
  username: String,
  password: String
}
pub async fn master_login(Json(master_login): Json<MasterLogin>) -> Result<Response<Body>, ApiError> {
  let res = db_authenticate_master(master_login.username.clone(), master_login.password).await?;

  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  Ok(
    match res {
      (StatusCode::OK, id) => response_builder.body(
        Body::from(
          json!(
            {
              "token": generate_token(master_login.username).await?,
              "id": id
            }
          ).to_string()
        ))?,
      (status_code, _) => response_builder.status(status_code).body(Body::from(""))?
    }
  )
}