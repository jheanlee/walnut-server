use argon2::Argon2;
use openssl::base64;
use crate::common::error::ApiError;

pub fn derive_key(salt: String, key: String, output: &mut [u8; 32]) -> Result<(), ApiError> {
  Argon2::default().hash_password_into(base64::decode_block(key.as_str())?.as_slice(), base64::decode_block(salt.as_str())?.as_slice(), output)?;
  Ok(())
}