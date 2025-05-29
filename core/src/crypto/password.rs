use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::aead::generic_array::GenericArray;
use argon2::Algorithm::Argon2id;
use argon2::Argon2;
use openssl::base64;
use openssl::rand::rand_bytes;
use crate::common::error::ApiError;
pub fn generate_salt() -> Result<String, ApiError>{
  let mut bytes: [u8; 16] = [0; 16];
  rand_bytes(&mut bytes)?;
  Ok(base64::encode_block(&bytes))
}

pub fn derive_key(salt: String, password: String, output: &mut [u8; 32]) -> Result<(), ApiError> {
  let res = Argon2::default().hash_password_into(password.as_bytes(), base64::decode_block(salt.as_str())?.as_slice(), output)?;
  Ok(())
}

pub fn aes_256_encrypt(key: &[u8; 32], nonce: &mut [u8; 12], plain_text: String) -> Result<Vec<u8>, ApiError> {
  let key = GenericArray::from_slice(key);
  let cipher = Aes256GcmSiv::new(&key);
  rand_bytes(nonce)?;
  let nonce = Nonce::from_slice(nonce);
  let cipher_text = cipher.encrypt(nonce, plain_text.as_bytes());
  Ok(cipher_text.map_err(|err| anyhow::anyhow!("{}", err.to_string()))?)
}

pub fn aes_256_decrypt(key: &[u8; 32], nonce: &[u8; 12], cipher_text: Vec<u8>) -> Result<Vec<u8>, ApiError> {
  let key = GenericArray::from_slice(key);
  let cipher = Aes256GcmSiv::new(&key);
  let nonce = Nonce::from_slice(nonce);
  let plain_text = cipher.decrypt(nonce, cipher_text.as_slice());
  Ok(plain_text.map_err(|err| anyhow::anyhow!("{}", err.to_string()))?)
}