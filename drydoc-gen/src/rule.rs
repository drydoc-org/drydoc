use super::config::Unit;
use super::page::Page;
use super::bundle::Bundle;
use url::Url;
use std::convert::From;

use async_trait::async_trait;

#[derive(Debug)]
pub enum Error {
  MissingRequiredParameter {
    param: String
  },
  InvalidParameter {
    param: String,
    message: String
  },
  InvalidInput {
    message: String
  },
  Io(tokio::io::Error)
}

impl From<tokio::io::Error> for Error {
  fn from(value: tokio::io::Error) -> Self {
    Self::Io(value)
  }
}

#[derive(Clone)]
pub struct Context {
  pub uri: Url
}

#[async_trait]
pub trait Rule {
  async fn apply(&self, unit: &Unit, context: &Context) -> Result<Bundle, Error>;
}