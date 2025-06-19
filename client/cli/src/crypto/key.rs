use openssl::base64;
use openssl::rand::rand_bytes;
use crate::common::error::Error;

pub fn generate_key() -> Result<String, Error> {
  let mut bytes = [0u8; 24];
  rand_bytes(&mut bytes)?;
  Ok(base64::encode_block(&bytes))
}

