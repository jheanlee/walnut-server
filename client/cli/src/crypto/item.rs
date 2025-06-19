use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::aead::generic_array::GenericArray;
use argon2::Argon2;
use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::sha::Sha256;
use crate::common::error::Error;
use crate::common::error::CryptoError;

pub fn encrypt_item(plaintext: String, key_base64: String) -> Result<String, Error> {
  let plaintext_bytes = plaintext.as_bytes();
  let key_bytes = base64::decode_block(key_base64.as_str())?;

  let mut message_key_bytes = [0u8; 32];
  derive_message_key(plaintext_bytes, &mut message_key_bytes)?;
  let message_key_base64 = base64::encode_block(&message_key_bytes);

  let mut aes_key_bytes = [0u8; 32];
  derive_aes_key(key_bytes.as_slice(), &message_key_bytes, &mut aes_key_bytes)?;

  let mut aes_iv_bytes= [0u8; 12];
  rand_bytes(&mut aes_iv_bytes)?;
  let aes_iv_base64 = base64::encode_block(&aes_iv_bytes);
  
  let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&aes_key_bytes));
  let encrypted = cipher.encrypt(Nonce::from_slice(&aes_iv_bytes), plaintext_bytes)
    .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;
  let encrypted = base64::encode_block(encrypted.as_slice());

  //  aes_iv (16B encoded), msg_key (44B encoded)
  let output = aes_iv_base64 + message_key_base64.as_str() + encrypted.as_str();
  Ok(output)
}

fn derive_message_key(plaintext_bytes: &[u8], output: &mut [u8]) -> Result<(), Error> {
  let mut salt = [0u8; 32];
  rand_bytes(&mut salt)?;
  let mut hasher = Sha256::new();
  hasher.update(plaintext_bytes);
  hasher.update(&salt);
  output.copy_from_slice(hasher.finish().as_ref());
  Ok(())
}

fn derive_aes_key(key_bytes: &[u8], message_key_bytes: &[u8], output: &mut [u8]) -> Result<(), Error> {
  Argon2::default().hash_password_into(key_bytes, message_key_bytes, output)?;
  Ok(())
}

pub fn decrypt_item(key_base64: String, data: String) -> Result<String, Error> {
  let key_bytes = base64::decode_block(key_base64.as_str())?;
  
  let aes_iv_base64 = data.get(0..=15)
    .ok_or_else(|| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  let aes_iv_bytes: [u8; 12] = base64::decode_block(aes_iv_base64)
    .map_err(|_| Error::CryptoError(CryptoError::DecryptInvalidFormat))?.as_slice().try_into()
    .map_err(|_| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  
  let message_key_base64 = data.get(16..=59)
    .ok_or_else(|| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  let message_key_bytes: [u8; 32] = base64::decode_block(message_key_base64)
    .map_err(|_| Error::CryptoError(CryptoError::DecryptInvalidFormat))?.as_slice().try_into()
    .map_err(|_| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  
  let encrypted_data_base64 = data.get(60..)
    .ok_or_else(|| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  let encrypted_data_bytes = base64::decode_block(encrypted_data_base64)
    .map_err(|_| Error::CryptoError(CryptoError::DecryptInvalidFormat))?;
  
  let mut aes_key_bytes = [0u8; 32];
  derive_aes_key(key_bytes.as_slice(), &message_key_bytes, &mut aes_key_bytes)?;

  let cipher = Aes256GcmSiv::new(GenericArray::from_slice(aes_key_bytes.as_slice()));
  let plaintext = cipher.decrypt(Nonce::from_slice(&aes_iv_bytes), encrypted_data_bytes.as_slice())
    .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;
  
  Ok(String::from_utf8(plaintext)?)
}