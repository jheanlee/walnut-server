use std::sync::Arc;
use axum::http::StatusCode;
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use crate::core::message::{api_message_type, Message};
use crate::error::ApiError;

pub async fn core_io_read_thread_func(unix_stream: Arc<Mutex<UnixStream>>) -> Result<(), ()> {
  let mut buffer = ['\0' as u8; 32768];
  let mut message = Message{ message_type: '\0' as u8, message_string: "".to_owned() };

  loop {
    loop {
      let sock_arc = Arc::clone(&unix_stream);
      let sock = sock_arc.lock().await;

      let status = tokio::select! {
        s = sock.readable() => Some(s),
        _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => None
      };

      if let Some(res) = status {
        if let Ok(_) = res {
          match sock.try_read(&mut buffer) {
            //  pipe closed
            Ok(n) if n == 0 => return Ok(()),
            //  data read
            Ok(_n) => break,
            //  no data available
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
            //  closed
            Err(ref e) if e.kind() == tokio::io::ErrorKind::BrokenPipe => {
              //  TODO logging pipe closed
              return Err(())
            },
            //  other errors
            Err(e) => {
              //  TODO logging general error
              return Err(())
            },
          }
        } else {
          //  TODO logging select error
        }
      }
    }

    if message.load(&buffer).is_err() {
      //  TODO logging invalid message
    }

    match message.message_type {
      api_message_type::API_HEARTBEAT => {
        send_heartbeat_message(Arc::clone(&unix_stream)).await.unwrap_or_else(|_| {
          //  TODO logging heartbeat failed
          StatusCode::INTERNAL_SERVER_ERROR
        });
      },
      _ => {}
    }

  }
}

pub async fn send_heartbeat_message(socket_core: Arc<Mutex<UnixStream>>) -> Result<StatusCode, ApiError> {
  let mut message = Message{ message_type: api_message_type::API_HEARTBEAT, message_string: "".to_owned() };

  loop {
    socket_core.lock().await.writable().await?;
    match socket_core.lock().await.try_write(message.dump()?.as_ref()) {
      Ok(_n) => break,
      Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
      Err(ref e) if e.kind() == tokio::io::ErrorKind::BrokenPipe => return Ok(StatusCode::SERVICE_UNAVAILABLE),
      Err(e) => return Err(e.into()),
    }
  }

  Ok(StatusCode::OK)
}