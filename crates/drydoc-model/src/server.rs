use super::*;

use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
  Log(Log),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRequest {
  pub context_id: u32,
  pub params: HashMap<String, String>,
  pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeRequest {
  pub version: u32,
  pub supported_encodings: HashSet<Encoding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenContextRequest {
  pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseContextRequest {
  pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestData {
  Initialize(InitializeRequest),
  OpenContext(OpenContextRequest),
  CloseContext(CloseContextRequest),
  Generate(GenerateRequest),
}

impl From<InitializeRequest> for RequestData {
  fn from(value: InitializeRequest) -> Self {
    Self::Initialize(value)
  }
}

impl From<OpenContextRequest> for RequestData {
  fn from(value: OpenContextRequest) -> Self {
    Self::OpenContext(value)
  }
}

impl From<CloseContextRequest> for RequestData {
  fn from(value: CloseContextRequest) -> Self {
    Self::CloseContext(value)
  }
}

impl From<GenerateRequest> for RequestData {
  fn from(value: GenerateRequest) -> Self {
    Self::Generate(value)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub id: u64,
  pub data: RequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseData {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub id: u64,
  pub data: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageData {
  Event(Event),
  Request(Request),
  Response(Response),
}

impl From<Event> for MessageData {
  fn from(value: Event) -> Self {
    Self::Event(value)
  }
}

impl From<Request> for MessageData {
  fn from(value: Request) -> Self {
    Self::Request(value)
  }
}

impl From<Response> for MessageData {
  fn from(value: Response) -> Self {
    Self::Response(value)
  }
}
