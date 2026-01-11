use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::{http, Json};
use axum::extract::{Query, Request};
use axum::middleware::Next;
use axum::response::IntoResponse;
use serde_json::json;
use crate::auth::jwt::generate_token;
use crate::common::error::ApiError;
use crate::CONFIG_CELL;
use crate::orm::master::{db_authenticate_master, db_delete_master, db_is_present, db_list_master, db_modify_master};
use crate::orm::password::db_delete_password_by_master;

pub async fn signup_availability_middleware(request: Request, next: Next) -> Result<axum::response::Response, StatusCode> {
  if CONFIG_CELL.get().unwrap().self_signup_enabled {
    let response = next.run(request).await;
    Ok(response)
  } else {
    Err(StatusCode::UNAUTHORIZED)
  }
}

pub async fn get_signup_availability() -> Result<impl IntoResponse, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let response_body = Body::from(serde_json::to_string(&json!({
    "signup_available" : CONFIG_CELL.get().unwrap().self_signup_enabled
  }))?);
  let response = response_builder.body(response_body)?;
  Ok(response)
}

pub async fn list_master() -> Result<Response<Body>, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  let master_items = db_list_master().await?;
  let response_body = Body::from(serde_json::to_string(&master_items)?);
  let response = response_builder.body(response_body)?;
  Ok(response)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IsUsernameAvailableQuery {
  username: String
}
pub async fn is_username_available(Query(query): Query<IsUsernameAvailableQuery>) -> Result<impl IntoResponse, ApiError> {
  let response_builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
  Ok(
    response_builder.body(json!({
      "available": !db_is_present(query.username).await?
    }).to_string())?
  )
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MasterSignup {
  username: String,
  password: String,
}
pub async fn master_signup(Json(master_signup): Json<MasterSignup>) -> Result<impl IntoResponse, ApiError> {
  let is_present = db_is_present(master_signup.username.clone()).await?;
  if !is_present {
    db_modify_master(master_signup.username, master_signup.password, false).await?;
    Ok(StatusCode::OK)
  } else {
    Ok(StatusCode::CONFLICT)
  }
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
              "token": generate_token(id.clone()).await?,
              "id": id
            }
          ).to_string()
        ))?,
      (status_code, _) => response_builder.status(status_code).body(Body::from(""))?
    }
  )
}