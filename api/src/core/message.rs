const MESSAGE_MAX_STRING_SIZE: usize = 126;

pub mod api_message_type {
  pub const API_CONNECT: u8 = 0x30;
  pub const API_EXIT: u8 = 0x31;
  pub const API_HEARTBEAT: u8 = 0x32;
}

pub struct Message {
  pub message_type: u8,
  pub message_string: String,
}

#[derive(Debug)]
pub enum MessageError {
  InvalidType,
  InvalidStringLength,
  InvalidString,
  InvalidBufferLength,
}

impl Message {
  pub fn dump(&mut self) -> Result<[u8; 128], MessageError> {
    if self.message_type == ('\0' as u8) {
      return Err(MessageError::InvalidType);
    }
    if self.message_string.len() > MESSAGE_MAX_STRING_SIZE {
      return Err(MessageError::InvalidStringLength);
    }
    if !self.message_string.is_ascii() {
      return Err(MessageError::InvalidString);
    }

    let mut buffer: [u8; 128] = [0; 128];

    buffer[0] = self.message_type.into();
    buffer[1..self.message_string.len() + 1].copy_from_slice(self.message_string.as_bytes());
    Ok(buffer)
  }

  pub fn load(&mut self, buffer: &[u8]) -> Result<(), MessageError> {
    if buffer.len() == 0 {
      return Err(MessageError::InvalidBufferLength);
    }

    if let Ok(str) = core::str::from_utf8(buffer) {
      self.message_type = str.chars().nth(0).unwrap_or('\0') as u8;
      self.message_string = str.chars().skip(1).filter(|c| c != &'\0').collect();
    } else {
      return Err(MessageError::InvalidString);
    }

    Ok(())
  }
}