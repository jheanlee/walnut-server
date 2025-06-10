use axum::routing::{get, post};
use clap::Parser;
use log::error;
use sea_orm::{Database, DatabaseConnection};
use crate::api::item::{delete_password_item, get_password_item, list_items, modify_password_item, new_password_item};
use crate::api::master::{list_master, modify_master, new_master};
use crate::common::opt::Args;
use crate::orm::tables::init_tables;

mod common;
mod api;
mod crypto;
mod orm;

pub struct Shared {
  pub database_connection: Option<DatabaseConnection>,
}

pub struct Config {
  
}


static SHARED_CELL: once_cell::sync::OnceCell<Shared> = once_cell::sync::OnceCell::new();
static CONFIG_CELL: once_cell::sync::OnceCell<Config> = once_cell::sync::OnceCell::new();

#[tokio::main]
async fn main() {
  let args = Args::parse();
  env_logger::Builder::from_default_env().filter_level(args.verbose_level()).init();

  SHARED_CELL.set(Shared {
    database_connection: Some(Database::connect(args.database).await.unwrap_or_else(|e| {
      error!("{}", e.to_string());
      panic!();
    })),
  }).unwrap_or_else(|_| {
    error!("Failed to set shared resources");
    panic!();
  });

  init_tables().await.unwrap_or_else(|e| {
    error!("{}", e.to_string());
    panic!();
  });

  let app = axum::Router::new()
    .route("/api/master/list", get(list_master))
    .route("/api/master/new", post(new_master))
    .route("/api/master/modify", post(modify_master))
    
    //  .layer(management_layer)
    
    .route("/api/items/list", get(list_items))
    .route("/api/items/password/new", post(new_password_item))
    .route("/api/items/password/modify", post(modify_password_item))
    .route("/api/items/password/delete", post(delete_password_item))
    .route("/api/items/password/get", get(get_password_item));
    //  .layer(authorisation_layer

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap_or_else(|e| {
    log::error!("{}", e.to_string());
    panic!();
  });
  
  axum::serve(listener, app).await.unwrap_or_else(|e| {
    log::error!("{}", e.to_string());
    panic!();
  });
}
