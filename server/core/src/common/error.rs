use axum::http::StatusCode;
use axum::response::Response;
use log::warn;

#[derive(Debug)]
pub enum ApiError {
  Error(anyhow::Error),
}

impl axum::response::IntoResponse for ApiError {
  fn into_response(self) -> Response {
    match self {
      ApiError::Error(e) => {
        warn!("{}", e.to_string());
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
      }
    }
  }
}

impl<E> From<E> for ApiError where E: Into<anyhow::Error> {
  fn from(error: E) -> Self {
    Self::Error(error.into())
  }
}

impl ApiError {
  pub fn to_string(self) -> String {
    match self {
        ApiError::Error(e) => e.to_string(),
    }
  }
}