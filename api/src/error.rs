use crate::core::message::MessageError;

#[derive(Debug)]
pub enum ApiError {
  Error(anyhow::Error),
  MessageError(MessageError)
}

impl<E> From<E> for ApiError where E: Into<anyhow::Error> {
  fn from(error: E) -> Self {
    Self::Error(error.into())
  }
}

impl From<MessageError> for ApiError {
  fn from(error: MessageError) -> Self {
    ApiError::MessageError(error)
  }
}