use crate::ns::Namespace;

use super::actor::{Actor, Addr, Receiver};
use super::config::Rule;

use std::{collections::HashMap, path::PathBuf};
use tokio::sync::oneshot::{Sender, channel};

use super::bundle::Bundle;

use std::sync::Arc;

use derive_more::*;

pub mod copy;
pub mod clang;
pub mod util;
pub mod ros;
pub mod rpc;


pub enum GeneratorsMsg {
  Get {
    name: String,
    sender: Sender<Option<Addr<GeneratorMsg>>>
  },
}

pub struct Generators {
  generators: HashMap<String, Addr<GeneratorMsg>>
}

impl Generators {
  pub fn new() -> Self {
    Self {
      generators: HashMap::new()
    }
  }

  pub async fn insert_generator<N, G>(&mut self, name: N, generator: G) -> Option<Addr<GeneratorMsg>>
  where
    N: Into<String>,
    G: Actor<Msg = GeneratorMsg>
  {
    self.generators.insert(name.into(), generator.spawn())
  }

  async fn run(self, mut rx: Receiver<GeneratorsMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorsMsg::Get { name, sender } => {
          let _ = sender.send(match self.generators.get(&name) {
            Some(addr) => Some(addr.clone()),
            None => None
          });
        }
      }
    }

    println!("Generators actor ended");
  }
}

impl Actor for Generators {
  type Msg = GeneratorsMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<GeneratorsMsg> {
  pub async fn get(&self, name: String) -> Option<Addr<GeneratorMsg>> {
    let (tx, rx) = channel();
    let _ = self.send(GeneratorsMsg::Get { name, sender: tx });
    rx.await.unwrap()
  }
}
#[derive(Display, Debug)]
pub enum GenerateError {
  MissingParameter(String),
  #[display(fmt = "InvalidParameter \"{}\": {}", name, message)]
  InvalidParameter {
    name: String,
    message: String
  },
  #[display(fmt = "Internal Error")]
  Internal(Box<dyn std::error::Error + Send>),
  Io(tokio::io::Error)
}

impl std::error::Error for GenerateError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Internal(err) => Some(err.as_ref()),
      Self::Io(err) => Some(err),
      _ => None
    }
  }
}

impl From<tokio::io::Error> for GenerateError {
  fn from(err: tokio::io::Error) -> Self {
    Self::Io(err)
  }
}

pub enum GeneratorMsg {
  Generate {
    rule: Rule,
    namespace: Arc<Namespace>,
    path: PathBuf,
    sender: Sender<Result<Bundle, GenerateError>>
  }
}

impl Addr<GeneratorMsg> {
  pub async fn generate(&self, rule: Rule, namespace: Arc<Namespace>, path: PathBuf) -> Result<Bundle, GenerateError> {
    let (tx, rx) = channel();
    let _ = self.send(GeneratorMsg::Generate {
      rule,
      namespace,
      path,
      sender: tx
    });
    rx.await.unwrap()
  }
}