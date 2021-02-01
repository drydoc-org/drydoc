use drydoc_pkg_manager::{Manager as PkgMgr, UrlFetcher, VersionReq};

use tokio::sync::oneshot::{channel, Sender};

use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::{
  actor::{Actor, Addr, Receiver},
  ipc::IpcMsg,
};

pub enum GeneratorMgrMsg {
  GetOrStart {
    name: String,
    version_req: VersionReq,
    res: Sender<Result<Addr<IpcMsg>, ()>>,
  },
}

pub struct GeneratorMgr {
  pkg_mgr: PkgMgr<UrlFetcher>,
  generators: HashMap<PathBuf, Addr<IpcMsg>>,
}

impl GeneratorMgr {
  pub fn new(pkg_mgr: PkgMgr<UrlFetcher>) -> Self {
    Self {
      pkg_mgr,
      generators: HashMap::new(),
    }
  }

  async fn get_or_start(
    &mut self,
    name: &str,
    version_req: &VersionReq,
  ) -> Result<Addr<IpcMsg>, Box<dyn Error>> {
    let (path, _, artifact) = self.pkg_mgr.get(name, &version_req).await?;

    if let Some(addr) = self.generators.get(&path) {
      Ok(addr.clone())
    } else {
      if let Some(gen) = artifact.as_generator() {
        let addr = crate::ipc::start_generator(&path, gen).await?;
        self.generators.insert(path, addr.clone());
        Ok(addr)
      } else {
        panic!("{:?} is not a generator", path);
      }
    }
  }

  async fn run(mut self, mut rx: Receiver<GeneratorMgrMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMgrMsg::GetOrStart {
          name,
          version_req,
          res,
        } => {
          res.send(
            self
              .get_or_start(name.as_str(), &version_req)
              .await
              .map_err(|_| ()),
          );
        }
      }
    }
  }
}

impl Actor for GeneratorMgr {
  type Msg = GeneratorMgrMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<GeneratorMgrMsg> {
  pub async fn get_or_start<N: Into<String>>(
    &self,
    name: N,
    version_req: VersionReq,
  ) -> Result<Addr<IpcMsg>, ()> {
    let (tx, rx) = channel();
    self.send(GeneratorMgrMsg::GetOrStart {
      name: name.into(),
      version_req,
      res: tx,
    });

    rx.await.unwrap()
  }
}
