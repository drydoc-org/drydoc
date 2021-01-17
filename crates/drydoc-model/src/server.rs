use super::*;

use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
  Log(Log)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateRequest {
  pub context_id: u32,
  pub params: HashMap<String, String>,
  pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeRequest {
  pub version: u32,
  pub supported_encodings: HashSet<Encoding>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenContextRequest {
  pub id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseContextRequest {
  pub id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestData {
  Initialize(InitializeRequest),
  OpenContext(OpenContextRequest),
  CloseContext(CloseContextRequest),
  Generate(GenerateRequest),
  
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub id: u64,
  pub data: RequestData
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseData {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  id: u64,
  data: ResponseData
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageData {
  Event(Event),
  Request(Request),
  Response(Response)
}