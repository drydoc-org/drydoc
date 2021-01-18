use crate::actor::{Actor, Addr, Receiver};
use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use super::GeneratorMsg;

use drydoc_ipc::MessageProcessor;

enum IpcGeneratorMsg {
  GeneratorMsg(GeneratorMsg),

}

impl From<GeneratorMsg> for IpcGeneratorMsg {
  fn from(value: GeneratorMsg) -> Self {
    Self::GeneratorMsg(value)
  }
}

pub struct IpcGenerator<R, W>
where
  R: 'static + AsyncRead + Send,
  W: 'static + AsyncWrite + Send
{
  write: W,
  read: Option<R>
}

impl<R, W> IpcGenerator<R, W>
where 
  R: AsyncRead + Send + Unpin,
  W: AsyncWrite + Send
{


  async fn read(addr: Addr<IpcGeneratorMsg>, mut read: R) {
    let addr = addr.downgrade();
    let mut processor = MessageProcessor::new();
    let mut buf = [0u8; 512];
    while let Ok(size) = read.read(&mut buf).await {
      if size == 0 {
        // FIXME: There's probably some way to not poll
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        continue;
      }

      let addr = addr.upgrade();
      processor.submit(&buf[..size]);
      while let Some(message) = processor.next() {
        
      }
    }
  }

  async fn run(self, mut rx: Receiver<IpcGeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      
    }
  }
}


impl<R, W> Actor for IpcGenerator<R, W>
where 
  R: AsyncRead + Send + Unpin,
  W: AsyncWrite + Send
{
  type Msg = GeneratorMsg;

  fn spawn(mut self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    let read = self.read.take().unwrap();
    tokio::spawn(self.run(rx));
    tokio::spawn(Self::read(addr.clone(), read));
    addr.upcast()
  }
}