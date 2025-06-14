use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct JwtKeyPair {
  pub encoding_key: EncodingKey,
  pub decoding_key: DecodingKey
}

#[derive(Debug)]
pub enum JwtKeyError {
  TokioError(tokio::io::Error),
  JwtError(jsonwebtoken::errors::Error)
}

impl From<jsonwebtoken::errors::Error> for JwtKeyError {
  fn from(error: jsonwebtoken::errors::Error) -> Self {
    Self::JwtError(error)
  }
}
impl From<tokio::io::Error> for JwtKeyError {
  fn from(error: tokio::io::Error) -> Self {
    Self::TokioError(error)
  }
}

impl JwtKeyError {
  pub fn to_string(self) -> String {
    match self { 
      Self::JwtError(e) => e.to_string(),
      Self::TokioError(e) => e.to_string()
    }
  }
}

pub async fn init_jwt_keys(private_key_path: &str, public_key_path: &str) -> Result<JwtKeyPair, JwtKeyError> {
  let priv_bytes = tokio::fs::read(private_key_path).await?;

  let encoding_key = EncodingKey::from_rsa_pem(priv_bytes.as_slice())
    .or_else(|_| EncodingKey::from_ec_pem(priv_bytes.as_slice()))
    .or_else(|_| EncodingKey::from_ed_pem(priv_bytes.as_slice()))?;

  let pub_bytes = tokio::fs::read(public_key_path).await?;

  let decoding_key = DecodingKey::from_rsa_pem(pub_bytes.as_slice())
    .or_else(|_| DecodingKey::from_ec_pem(pub_bytes.as_slice()))
    .or_else(|_| DecodingKey::from_ed_pem(pub_bytes.as_slice()))?;

  Ok(JwtKeyPair { encoding_key, decoding_key })
}