use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use core::io::core_io_read_thread_func;
use crate::core::connection::connect_core;

mod core;
mod error;

#[tokio::main]
async fn main() {
  let core_sock = connect_core().await.unwrap_or_else(|e| {
    //  TODO logging
    panic!();
  });
  let core_sock_arc = Arc::new(Mutex::new(core_sock));
  let core_io_thread = tokio::spawn(core_io_read_thread_func(Arc::clone(&core_sock_arc)));
  core_io_thread.await;
}
