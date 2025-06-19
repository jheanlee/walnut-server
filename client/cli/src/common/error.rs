#[derive(Debug)]
pub enum Error {
  Error(anyhow::Error),
  CryptoError(CryptoError),
  ApiError(ApiError)
}

#[derive(Debug)]
pub enum CryptoError {
  DecryptInvalidFormat
}

#[derive(Debug)]
pub enum ApiError {
  ExpiredToken,
  NoToken,
  InvalidToken,
}

impl<E> From<E> for Error
where E: Into<anyhow::Error> {
  fn from(error: E) -> Self {
    Self::Error(error.into())
  }
}

impl Error {
  pub fn to_string(&self) -> String {
    match self {
      Error::Error(e) => e.to_string(),
      Error::CryptoError(e) => e.to_string(),
      Error::ApiError(e) => e.to_string()
    }
  }
}

impl CryptoError {
  pub fn to_string(&self) -> String {
    match self {
      CryptoError::DecryptInvalidFormat => String::from("Invalid encrypted item format"),
    }
  }
}

impl ApiError {
  pub fn to_string(&self) -> String {
    match self {
      ApiError::ExpiredToken => String::from("Token has expired"),
      ApiError::NoToken => String::from("No token"),
      ApiError::InvalidToken => String::from("Invalid token"),
    }
  }
}

impl From<ApiError> for Error {
  fn from(error: ApiError) -> Error {
    Error::ApiError(error)
  }
}
