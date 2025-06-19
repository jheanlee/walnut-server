use serde_json::json;
use crate::{console, console_format, CONFIG_CELL, SHARED_CELL};
use crate::api::jwt::validify_token_expiry_prints;
use crate::common::console::Level;
use crate::common::error::{ApiError, Error};
use crate::services::client::result_handler;

#[derive(serde::Serialize, serde::Deserialize)]
struct TokenResponse {
  token: String
}
pub async fn login(username: String, password: String) -> Result<String, String> {
  let client = &SHARED_CELL.get().unwrap().client;

  let res = client.post(format!("http://{}/api/master/login", CONFIG_CELL.get().unwrap().host))
    .json(&json!({
      "username": username,
      "password": password
    })).send().await;
  match res {
    Ok(res) if res.status() == 200 => {
      let mut token = SHARED_CELL.get().unwrap().token.lock().await;
      *token = Some(
        serde_json::from_slice::<TokenResponse>(res.bytes().await.map_err(|e| {
          console_format!(Level::Error, "{}", e.to_string())
        })?.as_ref())
          .map_err(|e| {
            console_format!(Level::Error, "{}", e.to_string())
          })?
          .token
      );
      Ok(console_format!(Level::Info, "Successfully logged in"))
    }
    Ok(res) if res.status() == 401 => {
      Err(console_format!(Level::Error, "Incorrect username or password"))
    }
    Ok(res) if res.status() == 404 => {
      Err(console_format!(Level::Error, "Incorrect username or password"))
    }
    Ok(res) if res.status() == 500 => {
      Err(console_format!(Level::Error, "A serverside error has occurred"))
    }
    Ok(res) => {
      Err(console_format!(Level::Error, "Unexpected status from server, status code: {}", res.status()))
    }
    Err(e) => {
      Err(console_format!(Level::Error, "{}", e.to_string()))
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MasterItem {
  id: i32,
  username: String
}
pub async fn master_list() -> Result<(), String> {
  let client = &SHARED_CELL.get().unwrap().client;

  validify_token_expiry_prints().await?;

  let res = client.get(format!("http://{}/api/master/list", CONFIG_CELL.get().unwrap().host))
    .header("Authorization", SHARED_CELL.get().unwrap().token.lock().await.clone()
      .ok_or_else(|| {console_format!(Level::Error, "{}", ApiError::NoToken.to_string())})?)
    .send().await;
  
  match res {
    Ok(res) if res.status() == 200 => {
      //  serialise
      let data = serde_json::from_slice::<Vec<MasterItem>>(res.bytes().await.map_err(|e| {
        console_format!(Level::Error, "{}", e.to_string())
      })?.as_ref())
        .map_err(|e| {
          console_format!(Level::Error, "{}", e.to_string())
        })?;
      
      //  print
      let mut msg = String::from("\n");
      msg += " id      | user\n";
      msg += "---------|---------------------\n";
      for item in data {
        msg += format!(" {: <7} | {: <20}\n", item.id, item.username).as_str();
      }
      msg += "\n";
      
      let _printer = SHARED_CELL.get().unwrap().cmd_printer.lock()
        .map_err(|_| console_format!(Level::Error, "Lock poisoned"))?.as_mut()
        .print(msg);
    }
    Ok(res) if res.status() == 500 => {
      Err(console_format!(Level::Error, "A serverside error has occurred"))?
    }
    Ok(res) => {
      Err(console_format!(Level::Error, "Unexpected status from server, status code: {}", res.status()))?
    }
    Err(e) => {
      Err(console_format!(Level::Error, "{}", e.to_string()))?
    }
  }
  Ok(())
}

pub async fn master_new(username: String, password: String) -> Result<String, String> {
  let client = &SHARED_CELL.get().unwrap().client;

  validify_token_expiry_prints().await?;
  
  let res = client.post(format!("http://{}/api/master/new", CONFIG_CELL.get().unwrap().host))
    .header("Authorization", SHARED_CELL.get().unwrap().token.lock().await.clone()
      .ok_or_else(|| {console_format!(Level::Error, "{}", ApiError::NoToken.to_string())})?)
    .json(&json!({
      "username": username,
      "password": password
    }))
    .send().await;
  
  let handler_res = result_handler(&res);
  if handler_res.is_empty() {
    let msg: String = match res {
      Ok(res) if res.status() == 200 => 
        console_format!(Level::Info, "New user has been successfully created"),
      Ok(res) if res.status() == 409 => 
        console_format!(Level::Error, "A user with the same username exists"),
      Ok(res) if !matches!(res.status().as_u16(), 200 | 401 | 403 | 409 | 500..=599) => 
        console_format!(Level::Warning, "Unexpected status from server, status code: {}", res.status()),
      Ok(_) => { String::new() },
      Err(_) => { String::new() }
    };
    Ok(msg)
  } else {
    Ok(handler_res)
  }
}

pub async fn master_modify(username: String, password: String) -> Result<String, String> {
  let client = &SHARED_CELL.get().unwrap().client;

  validify_token_expiry_prints().await?;

  let res = client.post(format!("http://{}/api/master/modify", CONFIG_CELL.get().unwrap().host))
    .header("Authorization", SHARED_CELL.get().unwrap().token.lock().await.clone()
      .ok_or_else(|| {console_format!(Level::Error, "{}", ApiError::NoToken.to_string())})?)
    .json(&json!({
      "username": username,
      "password": password
    }))
    .send().await;

  let handler_res = result_handler(&res);
  if handler_res.is_empty() {
    let msg = match res {
      Ok(res) if res.status() == 200 => {
        console_format!(Level::Info, "User has been successfully modified")
      }
      Ok(res) if res.status() == 404 => {
        console_format!(Level::Error, "Could not find a user with this username")
      }
      Ok(res) if !matches!(res.status().as_u16(), 200 | 401 | 403 | 404 | 500..=599) => {
        console_format!(Level::Error, "Unexpected status from server, status code: {}", res.status())
      }
      Ok(_) => String::new(),
      Err(_) => String::new()
    };
    Ok(msg)
  } else {
    Ok(handler_res)
  }
}

pub async fn master_delete(username: String) -> Result<String, String> {
  let client = &SHARED_CELL.get().unwrap().client;

  validify_token_expiry_prints().await?;

  let res = client.post(format!("http://{}/api/master/delete", CONFIG_CELL.get().unwrap().host))
    .header("Authorization", SHARED_CELL.get().unwrap().token.lock().await.clone()
      .ok_or_else(|| {console_format!(Level::Error, "{}", ApiError::NoToken.to_string())})?)
    .json(&json!({
      "username": username,
    }))
    .send().await;

  let handler_res = result_handler(&res);
  if handler_res.is_empty() {
    let msg = match res {
      Ok(res) if res.status() == 200 => {
        console_format!(Level::Info, "User has been successfully deleted")
      }
      Ok(res) if res.status() == 404 => {
        console_format!(Level::Error, "Could not find a user with this username")
      }
      Ok(res) if !matches!(res.status().as_u16(), 200 | 401 | 403 | 404 | 500..=599) => {
        console_format!(Level::Error, "Unexpected status from server, status code: {}", res.status())
      }
      Ok(_) => String::new(),
      Err(_) => String::new()
    };
    Ok(msg)
  } else {
    Ok(handler_res)
  }
}