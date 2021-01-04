use super::actor::{Actor, Addr, Receiver};
use super::config::Unit;

use std::collections::HashMap;
use tokio::sync::oneshot::{Sender, channel};

use super::bundle::Bundle;

pub mod copy;
pub mod clang;


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
#[derive(Debug)]
pub enum GenerateError {
  MissingParameter(String),
  InvalidParameter {
    name: String,
    message: String
  },
  Internal(Box<dyn std::error::Error + Send>),
  Io(tokio::io::Error)
}

impl From<tokio::io::Error> for GenerateError {
  fn from(err: tokio::io::Error) -> Self {
    Self::Io(err)
  }
}

pub enum GeneratorMsg {
  Generate {
    unit: Unit,
    prefix: String,
    sender: Sender<Result<Bundle, GenerateError>>
  }
}

impl Addr<GeneratorMsg> {
  pub async fn generate(&self, unit: Unit, prefix: String) -> Result<Bundle, GenerateError> {
    let (tx, rx) = channel();
    let _ = self.send(GeneratorMsg::Generate {
      unit,
      prefix,
      sender: tx
    });
    rx.await.unwrap()
  }
}