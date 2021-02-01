use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel {
  Verbose,
  Debug,
  Info,
  Warning,
  Error,
  Fatal,
}

impl LogLevel {
  pub fn level(&self) -> u8 {
    match self {
      Self::Verbose => 0,
      Self::Debug => 1,
      Self::Info => 2,
      Self::Warning => 3,
      Self::Error => 4,
      Self::Fatal => 5,
    }
  }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
  level: LogLevel,
  message: String,
}
