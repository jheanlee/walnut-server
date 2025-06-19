use crate::common::console::Level;
use crate::console_format;

pub fn result_handler(result: &Result<reqwest::Response, reqwest::Error>) -> String {
  match result {
    Ok(response) if response.status() == 401 => {
      console_format!(Level::Error, "Login expired")
    }
    Ok(response) if response.status() == 403 => {
      console_format!(Level::Error, "Access denied")
    }
    Ok(response) if (500..=599).contains(&response.status().as_u16()) => {
      console_format!(Level::Error, "A serverside error has occurred")
    }
    Err(error) => {
      console_format!(Level::Error, "{}", error.to_string())
    }
    _ => "".to_string()
  }
}