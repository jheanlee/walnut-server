use std::mem::needs_drop;
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::Nonce;

mod common;
mod api;
mod crypto;
mod orm;

#[tokio::main]
async fn main() {

}
