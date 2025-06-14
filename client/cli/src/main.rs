use clap::Parser;
use crate::common::opt::Args;

mod common;
mod connection;
mod crypto;

#[tokio::main]
async fn main() {
  let args = Args::parse();
  
  
}