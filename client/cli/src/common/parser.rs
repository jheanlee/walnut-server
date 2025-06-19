use std::process::exit;
use clap::Command;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::api::master::{master_delete, master_list, master_modify, master_new};
use crate::{console_format, SHARED_CELL};
use crate::common::console::Level;

fn println(msg: String) -> Result<(), String>{
  SHARED_CELL.get().unwrap().cmd_printer.lock()
    .map_err(|_| console_format!(Level::Error, "Lock poisoned"))?.as_mut()
    .print(msg + "\n").map_err(|e| e.to_string())?;
  Ok(())
}

pub async fn parse_line(line: &str) -> Result<(), String> {
  let args = shlex::split(line)
    .ok_or("Invalid quoting")?;

  let cli = Command::new("repl")
    .multicall(true)
    .arg_required_else_help(true)
    .subcommand_required(true)
    .subcommand(
      Command::new("master")
        .about("Master user related actions")
        .subcommand_required(true)
        .subcommand(Command::new("list"))
        .subcommand(Command::new("new"))
        .subcommand(Command::new("modify"))
        .subcommand(Command::new("delete"))
    )
    .subcommand(
      Command::new("exit")
        .alias("quit")
        .about("Exit the program")
    )
    .try_get_matches_from(args).map_err(|e| e.to_string());

  if let Err(error) = cli {
    SHARED_CELL.get().unwrap().cmd_printer.lock()
      .map_err(|_| console_format!(Level::Error, "Lock poisoned"))?.as_mut()
      .print(error).map_err(|e| e.to_string())?;
    return Ok(());
  }
  let cli = cli.unwrap();

  match cli.subcommand() {
    Some(("exit", _exit)) => exit(0),

    Some(("master", master)) => {
      match master.subcommand() {
        Some(("list", _list)) => {
          match master_list().await {
            Ok(_) => {}
            Err(err) => {
              println(err)?;
            }
          }
        }

        Some(("new", _new)) => {
          let mut username = String::new();
          let mut password = String::new();
          let mut rustyline = DefaultEditor::new().map_err(|e| e.to_string())?;
          //  username loop
          loop {
            let line = rustyline.readline("New username: ");
            match line {
              Ok(line) => {
                username = line.trim().to_string();
                if !username.is_empty() {
                  break;
                }
              }
              Err(e) => {
                match e {
                  ReadlineError::Eof => break,
                  ReadlineError::Interrupted => break,
                  e => Err(e.to_string())?
                }
              }
            }
          }
          if username.is_empty() {
            return Ok(());
          }

          //  password loop
          loop {  //  TODO password masking
            let line = rustyline.readline("New password: ");
            match line {
              Ok(line) => {
                password = line.trim().to_string();
                if !password.is_empty() {
                  break;
                }
              }
              Err(e) => {
                match e {
                  ReadlineError::Eof => break,
                  ReadlineError::Interrupted => break,
                  e => Err(e.to_string())?
                }
              }
            }
          }
          if password.is_empty() {
            return Ok(());
          }

          match master_new(username, password).await {
            Ok(res) => {
              println(res)?;
            }
            Err(err) => {
              println(err)?;
            }
          }
        }
        Some(("modify", _modify)) => {
          let mut username = String::new();
          let mut password = String::new();
          let mut rustyline = DefaultEditor::new().map_err(|e| e.to_string())?;
          //  username loop
          loop {
            let line = rustyline.readline("User to modify: ");
            match line {
              Ok(line) => {
                username = line.trim().to_string();
                if !username.is_empty() {
                  break;
                }
              }
              Err(e) => {
                match e {
                  ReadlineError::Eof => break,
                  ReadlineError::Interrupted => break,
                  e => Err(e.to_string())?
                }
              }
            }
          }
          if username.is_empty() {
            return Ok(());
          }

          //  password loop
          loop {  //  TODO password masking
            let line = rustyline.readline("New password: ");
            match line {
              Ok(line) => {
                password = line.trim().to_string();
                if !password.is_empty() {
                  break;
                }
              }
              Err(e) => {
                match e {
                  ReadlineError::Eof => break,
                  ReadlineError::Interrupted => break,
                  e => Err(e.to_string())?
                }
              }
            }
          }
          if password.is_empty() {
            return Ok(());
          }

          match master_modify(username, password).await {
            Ok(res) => {
              println(res)?;
            }
            Err(err) => {
              println(err)?;
            }
          }
        }
        Some(("delete", _delete)) => {
          let mut username = String::new();
          let mut rustyline = DefaultEditor::new().map_err(|e| e.to_string())?;
          //  username loop
          loop {
            let line = rustyline.readline("User to delete: ");
            match line {
              Ok(line) => {
                username = line.trim().to_string();
                if !username.is_empty() {
                  break;
                }
              }
              Err(e) => {
                match e {
                  ReadlineError::Eof => break,
                  ReadlineError::Interrupted => break,
                  e => Err(e.to_string())?
                }
              }
            }
          }
          if username.is_empty() {
            return Ok(());
          }
          
          match master_delete(username).await {
            Ok(res) => { 
              println(res)?;
            }
            Err(err) => {
              println(err)?;
            }
          }
        }
        Some(_) => unimplemented!(),
        None => unreachable!()
      }
    }
    Some(_) => unimplemented!(),
    None => unreachable!()
  }
  Ok(())
}
