pub struct RealFile {
  
}

impl RealFile {
  pub fn new<N: Into<String>, C: Into<Arc<[u8]>>>(name: N, contents: C) -> Self {
    Self {
      name: name.into(),
      contents: contents.into()
    }
  }

  async fn run(mut self, mut rx: Receiver<FileMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        FileMsg::Get { sender } => {
          let _ = sender.send((self.name.clone(), self.contents.clone()));
        }
      }
    }
  }
}

impl Actor for File {
  type Msg = FileMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}