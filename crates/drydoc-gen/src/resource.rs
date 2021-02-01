use super::uri::Uri;

use super::actor::{Actor, Addr, SendError};
use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::{channel, Receiver, Sender};

pub enum GetError {
  Io(tokio::io::Error),
  Receive,
  Send(SendError),
}

impl From<tokio::io::Error> for GetError {
  fn from(err: tokio::io::Error) -> Self {
    Self::Io(err)
  }
}

impl From<SendError> for GetError {
  fn from(err: SendError) -> Self {
    Self::Send(err)
  }
}

pub enum ResourceMsg<T>
where
  T: Send + Sync,
{
  Get(Sender<Result<T, GetError>>),
}

pub trait Resource<T>: Actor<Msg = T>
where
  T: Send + Sync,
{
}

impl<T> Addr<ResourceMsg<T>>
where
  T: 'static + Send + Sync,
{
  pub async fn get(&self) -> Result<T, GetError> {
    let (tx, rx) = channel();
    match self.send(ResourceMsg::Get(tx)) {
      Err(err) => return Err(GetError::Send(err)),
      _ => {}
    }

    match rx.await {
      Ok(v) => v,
      Err(_) => Err(GetError::Receive),
    }
  }
}

pub struct UriResource {
  uri: Uri,
  contents: Option<Arc<[u8]>>,
}

impl UriResource {
  pub fn new(uri: Uri) -> Self {
    Self {
      uri,
      contents: None,
    }
  }

  async fn get(&mut self) -> tokio::io::Result<Arc<[u8]>> {
    if let Some(contents) = &self.contents {
      Ok(contents.clone())
    } else {
      let contents: Arc<[u8]> = super::fetch::fetch(&self.uri).await?.into();
      self.contents = Some(contents.clone());
      Ok(contents)
    }
  }

  async fn run(mut self, mut rx: UnboundedReceiver<ResourceMsg<Arc<[u8]>>>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        ResourceMsg::Get(sender) => {
          let _ = sender.send(self.get().await.map_err(|err| err.into()));
        }
      }
    }
  }
}

impl Actor for UriResource {
  type Msg = ResourceMsg<Arc<[u8]>>;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

pub struct TransformResource<T, U, F>
where
  T: Send + Sync,
  U: Send + Sync + Clone,
  F: Fn(T) -> U + Send,
{
  underlying: Addr<ResourceMsg<T>>,
  transformer: F,
  value: Option<U>,
}

impl<T, U, F> TransformResource<T, U, F>
where
  T: 'static + Send + Sync,
  U: Send + Sync + Clone,
  F: Fn(T) -> U + Send,
{
  pub fn wrap<A: Into<Addr<ResourceMsg<T>>>, G: Into<F>>(underlying: A, transformer: G) -> Self {
    Self {
      underlying: underlying.into(),
      transformer: transformer.into(),
      value: None,
    }
  }

  async fn run(mut self, mut rx: UnboundedReceiver<ResourceMsg<U>>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        ResourceMsg::Get(sender) => {
          if let Some(value) = &self.value {
            let _ = sender.send(Ok(value.clone()));
          } else {
            match self.underlying.get().await {
              Ok(value) => {
                let value = (self.transformer)(value);
                self.value = Some(value.clone());
                let _ = sender.send(Ok(value));
              }
              Err(err) => {
                let _ = sender.send(Err(err));
              }
            }
          }
        }
      }
    }
  }
}

impl<T, U, F> Actor for TransformResource<T, U, F>
where
  T: 'static + Send + Sync,
  U: 'static + Send + Sync + Clone,
  F: 'static + Fn(T) -> U + Send,
{
  type Msg = ResourceMsg<U>;

  fn spawn(mut self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}
