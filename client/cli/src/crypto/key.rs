use openssl::base64;
use openssl::rand::rand_bytes;
use crate::common::error::ApiError;

pub fn generate_key() -> Result<String, ApiError> {
  let mut bytes = [0u8; 24];
  rand_bytes(&mut bytes)?;
  Ok(base64::encode_block(&bytes))
}

