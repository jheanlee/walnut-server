use std::ops::DerefMut;
use crate::SHARED_CELL;

pub enum Level {
  Critical,
  Error,
  Warning,
  Info,
  Debug
}
pub fn format_console(level: Level, args: std::fmt::Arguments) -> String {
  let msg = match level {
    Level::Critical => format!("\x1b[31m[Critical]\x1b[0m {}", args),
    Level::Error => format!("\x1b[31m[Error]\x1b[0m {}", args),
    Level::Warning => format!("\x1b[33m[Warning]\x1b[0m {}", args),
    Level::Info => format!("[Info] {}", args),
    Level::Debug => format!("\x1b[2;90m[Debug]\x1b[0m {}", args)
  };
  
  msg
}

pub fn print_console(level: Level, args: std::fmt::Arguments) {
  let msg = match level {
    Level::Critical => format!("\x1b[31m[Critical]\x1b[0m {}", args),
    Level::Error => format!("\x1b[31m[Error]\x1b[0m {}", args),
    Level::Warning => format!("\x1b[33m[Warning]\x1b[0m {}", args),
    Level::Info => format!("[Info] {}", args),
    Level::Debug => format!("\x1b[2;90m[Debug]\x1b[0m {}", args)
  };

  SHARED_CELL.get().unwrap().cmd_printer.lock()
    .unwrap_or_else(|_| {
      println!("\x1b[31m[Error]\x1b[0m Lock poisoned");
      panic!();
    }).deref_mut()
    .print(msg + "\n").unwrap_or_else(|e| {
    println!("\x1b[31m[Error]\x1b[0m {}", e.to_string());
    panic!();
  });
}

#[macro_export]
macro_rules! console {
    ($level:expr, $($arg:tt)*) => {
        $crate::common::console::print_console($level, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! console_format {
    ($level:expr, $($arg:tt)*) => {
        $crate::common::console::format_console($level, format_args!($($arg)*))
    };
}