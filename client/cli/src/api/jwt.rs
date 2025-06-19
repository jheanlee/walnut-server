use std::time::{SystemTime, UNIX_EPOCH};
use crate::common::error::{ApiError, Error};
use crate::{console, console_format, SHARED_CELL};
use crate::common::console::Level;

#[derive(serde::Deserialize)]
struct Claims {
  sub: String,
  exp: u64,
  iat: u64
}
pub async fn validify_token_expiry() -> Result<(), Error> {
  let token = SHARED_CELL.get().unwrap().token.lock().await.clone().ok_or_else(|| ApiError::NoToken)?;
  let payload = token.split('.').nth(1).ok_or_else(|| ApiError::InvalidToken)?;
  let decoded = base64_url::decode(payload)?;
  let exp = serde_json::from_slice::<Claims>(decoded.as_slice())?.exp;
  if SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() > exp {
    Err(ApiError::ExpiredToken.into())
  } else {
    Ok(())
  }
}

pub async fn validify_token_expiry_prints() -> Result<(), String> {
  validify_token_expiry().await.map_err(|e| {
    console_format!(Level::Error, "{}", e.to_string())
  })
}