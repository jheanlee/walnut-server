use std::process::exit;
use clap::Parser;
use reqwest::Client;
use rustyline::{DefaultEditor, ExternalPrinter};
use rustyline::error::ReadlineError;
use tokio::sync::Mutex;
use crate::api::master::login;
use crate::common::console::Level;
use crate::common::opt::Args;
use crate::common::parser::parse_line;

mod common;
mod connection;
mod crypto;
mod api;
mod services;

pub struct Shared {
  token: Mutex<Option<String>>,
  client: Client,
  cmd_printer: std::sync::Mutex<Box<dyn ExternalPrinter + Send + Sync + 'static>>
}

pub struct Config {
  host: String
}

static SHARED_CELL: std::sync::OnceLock<Shared> = std::sync::OnceLock::new();
static CONFIG_CELL: once_cell::sync::OnceCell<Config> = once_cell::sync::OnceCell::new();

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let mut rustyline = DefaultEditor::new().unwrap_or_else(|_| {
    println!("\x1b[31m[Critical]\x1b[0m Failed to initialise input service");
    panic!();
  });
  let mut printer = rustyline.create_external_printer().unwrap_or_else(|_| {
    println!("\x1b[31m[Critical]\x1b[0m Failed to initialise input service");
    panic!();
  });

  SHARED_CELL.set(Shared {
    token: Mutex::new(None),
    client: Client::new(),
    cmd_printer: std::sync::Mutex::new(Box::new(printer))
  }).unwrap_or_else(|_| {
    console!(Level::Critical, "Failed to set shared resources");
    panic!();
  });

  CONFIG_CELL.set(Config {
    host: args.host
  }).unwrap_or_else(|_| {
    console!(Level::Critical, "Failed to set config resources");
    panic!();
  });

  //  login loop
  loop {
    let mut username = String::new();
    let mut password = String::new();
    
    //  username loop
    loop {
      let line = rustyline.readline("Username: ");
      match line {
        Ok(line) => {
          username = line.trim().to_string();
          if !username.is_empty() {
            break;
          }
        }
        Err(e) => {
          match e {
            ReadlineError::Eof => {
              exit(0);
            }
            ReadlineError::Interrupted => {
              exit(130);
            }
            _ => {
              console!(Level::Error, "{}", e.to_string());
            }
          }
        }
      }
    }

    //  password loop
    loop {
      let line = rustyline.readline("Password: ");
      match line {
        Ok(line) => {
          password = line.trim().to_string();
          if !password.is_empty() {
            break;
          }
        }
        Err(e) => {
          match e {
            ReadlineError::Eof => {
              break;
            }
            ReadlineError::Interrupted => {
              break;
            }
            _ => {
              console!(Level::Error, "{}", e.to_string());
            }
          }
        }
      }
    }
    if password.is_empty() {
      continue;
    }

    //  login
    match login(username, password).await {
      Ok(res) => {
        SHARED_CELL.get().unwrap().cmd_printer.lock()
          .unwrap_or_else(|e| { console!(Level::Error, "Mutex poisoned"); panic!(); })
          .print(res + "\n").unwrap_or_else(|e| { console!(Level::Error, "{}", e.to_string()); });
        break;
      }
      Err(err) => {
        SHARED_CELL.get().unwrap().cmd_printer.lock()
          .unwrap_or_else(|e| { console!(Level::Error, "Mutex poisoned"); panic!(); })
          .print(err + "\n").unwrap_or_else(|e| { console!(Level::Error, "{}", e.to_string()); });
      }
    }
  }

  loop {
    let line = rustyline.readline("> ").unwrap_or_else(|e| {
      match e {
        ReadlineError::Eof => {
          exit(0);
        }
        ReadlineError::Interrupted => {
          exit(130);
        }
        _ => {
          console!(Level::Error, "{}", e.to_string());
          "".to_string()
        }
      }
    });
    rustyline.add_history_entry(line.as_str()).unwrap_or_else(|e| {
      match e {
        ReadlineError::Eof => {
          exit(0);
        }
        ReadlineError::Interrupted => {
          exit(130);
        }
        _ => {
          console!(Level::Error, "{}", e.to_string());
          false
        }
      }
    });

    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    match parse_line(line).await {
      Ok(_) => {}
      Err(e) => { console!(Level::Error, "{}", e); }
    }
  }
}