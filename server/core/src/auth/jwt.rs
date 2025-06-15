use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{get_current_timestamp, Algorithm, Header, Validation};
use crate::common::error::ApiError;
use crate::SHARED_CELL;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
  pub sub: String,
  pub exp: u64,
  pub iat: u64,
}

pub async fn generate_token(sub: String) -> Result<String, ApiError> {
  let claims = Claims {
    sub: sub,
    iat: get_current_timestamp(),
    exp: get_current_timestamp() + 600,
  };
  let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &SHARED_CELL.get().unwrap().jwt_key_pair.encoding_key)?;
  Ok(token)
}

pub async fn verify_token(header_map: HeaderMap, request: Request, next: Next) -> Result<Response, StatusCode> {
  if let Some(token) = header_map.get("authorization") {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["sub", "iat", "exp"]);

    match jsonwebtoken::decode::<Claims>(token.to_str().unwrap_or_else(|_| {""}), &SHARED_CELL.get().unwrap().jwt_key_pair.decoding_key, &validation) {
      Ok(_) => {
        let response = next.run(request).await;
        Ok(response)
      },
      Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
  } else {
    Err(StatusCode::UNAUTHORIZED)
  }
}