use crate::ns::Namespace;

use super::actor::{Actor, Addr, Receiver};
use super::config::Rule;

use std::{collections::HashMap, path::PathBuf};
use tokio::{io::{AsyncRead, AsyncWrite}, sync::oneshot::{Sender, channel}};

use super::bundle::Bundle;

use crate::ipc::{IpcMsg, pipe};

use drydoc_pkg_manager::{Artifact, Fetcher, Manager as PkgMgr, Version, VersionReq};

use std::sync::Arc;

use derive_more::*;

pub mod copy;
pub mod clang;
pub mod util;
pub mod ros;
pub mod rpc;

use tokio::process::Command;
use std::process::Stdio;
pub enum GeneratorsMsg {
  Get {
    name: String,
    version_req: VersionReq,
    sender: Sender<Option<Addr<IpcMsg>>>
  },
}

pub struct Generators<F>
where
  F: Fetcher + Send + Sync
{
  pkg_mgr: PkgMgr<F>,
  generators: HashMap<String, Vec<(Version, Addr<IpcMsg>)>>
}

impl<F> Generators<F>
where
  F: Fetcher + Send + Sync
{
  pub fn new(pkg_mgr: PkgMgr<F>) -> Self {

    Self {
      pkg_mgr,
      generators: HashMap::new()
    }
  }

  async fn get(&mut self, name: String, version_req: VersionReq) -> Option<Addr<IpcMsg>> {
    if let Some(versions) = self.generators.get(&name) {
      for (version, addr) in versions.iter() {
        if version_req.matches(&version.clone().into()) {
          return Some(addr.clone());
        }
      }
    }
    
    let (mut path, version, artifact) = self.pkg_mgr.get(name.as_str(), &version_req).await.unwrap();

    if let Artifact::Generator(gen) = artifact {

      path.push(gen.entrypoint);
      let cmd = Command::new(&path)
        .stdout(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
      let addr = pipe(cmd.stdout.unwrap(), cmd.stdin.unwrap()).await;
      let entry = (version, addr.clone());
      if let Some(gen) = self.generators.get_mut(&name) {
        gen.push(entry);
      } else {
        self.generators.insert(name, vec![ entry ]);
      }
      Some(addr)
    } else {
      panic!("Expected generator artifact")
    }
  }

  async fn run(mut self, mut rx: Receiver<GeneratorsMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorsMsg::Get { name, version_req, sender } => {
          let _ = sender.send(self.get(name, version_req).await);
        }
      }
    }

    println!("Generators actor ended");
  }
}

impl<F> Actor for Generators<F>
where
  F: 'static + Fetcher + Send + Sync
{
  type Msg = GeneratorsMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<GeneratorsMsg> {
  pub async fn get(&self, name: String, version_req: VersionReq) -> Option<Addr<IpcMsg>> {
    let (tx, rx) = channel();
    let _ = self.send(GeneratorsMsg::Get { name, version_req, sender: tx });
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

