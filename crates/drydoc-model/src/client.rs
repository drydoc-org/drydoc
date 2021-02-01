use super::*;

use bundle::Bundle;
use fs::LinkedFileHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressEvent {
  pub context: u32,
  pub job: u32,
  pub completion: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
  Progress(ProgressEvent),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenRequest {
  pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseRequest {
  pub handle: LinkedFileHandle,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestData {
  Open(OpenRequest),
  Release(ReleaseRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub id: u64,
  pub data: RequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResponse {
  pub encoding: Encoding,
  pub requires_direct_fs_access: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenContextResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseContextResponse {
  pub bundle: Option<Bundle>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResponse {
  pub bundle: Bundle,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseData {
  Initialize(InitializeResponse),
  OpenContext(OpenContextResponse),
  CloseContext(CloseContextResponse),
  Generate(GenerateResponse),
}

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
