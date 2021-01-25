use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use std::sync::{Arc, Weak};

pub type Receiver<T> = UnboundedReceiver<T>;

pub mod map;

use derive_more::{Display, Error};

#[derive(Display, Debug, Error)]
pub enum SendError {
  Internal,
  RecvError
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for SendError {
  fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
    Self::Internal
  }
}

impl From<tokio::sync::oneshot::error::RecvError> for SendError {
  fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
    Self::RecvError
  }
}

trait AddrImpl<T>: Send + Sync
where
  T: Send + Sync
{
  fn send(&self, message: T) -> Result<(), SendError>; 
}

struct DefaultAddrImpl<T> {
  tx: UnboundedSender<T>
}

impl<T> DefaultAddrImpl<T>
where
  T: Send + Sync
{
  pub fn new() -> (Self, Receiver<T>) {
    let (tx, rx) = unbounded_channel();
    (Self { tx }, rx)
  }
}

impl<T> AddrImpl<T> for DefaultAddrImpl<T>
where
  T: Send + Sync
{
  fn send(&self, message: T) -> Result<(), SendError> {
    Ok(self.tx.send(message)?)
  }
}

struct CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Into<T> + Sync
{
  imp: Arc<dyn AddrImpl<T> + Send>,
  phantom: std::marker::PhantomData<U>
}

impl<T, U> CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Into<T> + Sync
{
  pub fn new(imp: Arc<dyn AddrImpl<T> + Send>) -> Self {
    Self {
      imp,
      phantom: std::marker::PhantomData {}
    }
  }
}

impl<T, U> AddrImpl<U> for CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Sync + Into<T>
{
  fn send(&self, message: U) -> Result<(), SendError> {
    self.imp.send(message.into())
  }
}

pub struct Addr<T>
  where T: Send + Sync
{
  imp: Arc<dyn AddrImpl<T> + Send + Sync>
}

impl<T> Clone for Addr<T>
where
  T: Send + Sync
{
  fn clone(&self) -> Self {
    Self {
      imp: self.imp.clone()
    }
  }
}

impl<T> Addr<T>
where
  T: 'static + Send + Sync
{
  pub fn new() -> (Self, Receiver<T>) {
    let (imp, rx) = DefaultAddrImpl::new();
    (Self { imp: Arc::new(imp) }, rx)
  }

  pub fn upcast<U>(&self) -> Addr<U>
  where
    U: 'static + Send + Into<T> + Sync
  {
    Addr {
      imp: Arc::new(CastAddrImpl::new(self.imp.clone()))
    }
  }

  pub fn send<V: Into<T>>(&self, message: V) -> Result<(), SendError> {
    self.imp.send(message.into())
  }

  pub fn downgrade(&self) -> WeakAddr<T> {
    WeakAddr { imp: Arc::downgrade(&self.imp) }
  }
}

pub struct WeakAddr<T>
  where T: Send + Sync
{
  imp: Weak<dyn AddrImpl<T> + Send + Sync>
}

impl<T> Clone for WeakAddr<T>
where
  T: Send + Sync
{
  fn clone(&self) -> Self {
    Self {
      imp: self.imp.clone()
    }
  }
}

impl<T> WeakAddr<T>
  where T: Send + Sync
{
  pub fn upgrade(&self) -> Option<Addr<T>> {
    self.imp.upgrade().map(|imp| Addr { imp })
  }
}

pub trait Actor {
  type Msg: Send + Sync;
  fn spawn(self) -> Addr<Self::Msg>;
}

impl<A> From<A> for Addr<A::Msg>
where
  A: Actor
{
  fn from(actor: A) -> Self {
    actor.spawn()
  }
}