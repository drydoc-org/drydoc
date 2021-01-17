use serde::{Serialize, Deserialize};

pub enum LogLevel {
  Verbose,
  Debug,
  Info,
  Warning,
  Error,
  Fatal  
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

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
  Log()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRequest {
  
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  Generate(GenerateRequest)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResponse {

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
  Generate(GenerateResponse)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageData {
  Event(),
  Request(),
  Response()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  id: u64,
  data: MessageData
}