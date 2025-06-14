#[derive(Debug)]
pub enum ApiError {
  Error(anyhow::Error),
  CryptoError(CryptoError),
}

#[derive(Debug)]
pub enum CryptoError {
  DecryptInvalidFormat
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
      ApiError::CryptoError(e) => e.to_string(),
    }
  }
}

impl CryptoError {
  pub fn to_string(self) -> String {
    match self {
        CryptoError::DecryptInvalidFormat => String::from("invalid encrypted item format"),
    }
  }
}