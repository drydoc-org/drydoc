use std::sync::{Arc, Weak};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub type Receiver<T> = UnboundedReceiver<T>;

pub mod map;
pub mod store;

use derive_more::{Display, Error};

#[derive(Display, Debug, Error)]
pub enum SendError {
  Internal,
  RecvError,
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
  T: Send + Sync,
{
  fn send(&self, message: T) -> Result<(), SendError>;
}

struct DefaultAddrImpl<T> {
  tx: UnboundedSender<T>,
}

impl<T> DefaultAddrImpl<T>
where
  T: Send + Sync,
{
  pub fn new() -> (Self, Receiver<T>) {
    let (tx, rx) = unbounded_channel();
    (Self { tx }, rx)
  }
}

impl<T> AddrImpl<T> for DefaultAddrImpl<T>
where
  T: Send + Sync,
{
  fn send(&self, message: T) -> Result<(), SendError> {
    Ok(self.tx.send(message)?)
  }
}

struct CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Into<T> + Sync,
{
  imp: Arc<dyn AddrImpl<T> + Send>,
  phantom: std::marker::PhantomData<U>,
}

impl<T, U> CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Into<T> + Sync,
{
  pub fn new(imp: Arc<dyn AddrImpl<T> + Send>) -> Self {
    Self {
      imp,
      phantom: std::marker::PhantomData {},
    }
  }
}

impl<T, U> AddrImpl<U> for CastAddrImpl<T, U>
where
  T: Send + Sync,
  U: Send + Sync + Into<T>,
{
  fn send(&self, message: U) -> Result<(), SendError> {
    self.imp.send(message.into())
  }
}

/// A strong reference to an actor that accepts messages of type `T`.
pub struct Addr<T>
where
  T: Send + Sync,
{
  imp: Arc<dyn AddrImpl<T> + Send + Sync>,
}

impl<T> Clone for Addr<T>
where
  T: Send + Sync,
{
  fn clone(&self) -> Self {
    Self {
      imp: self.imp.clone(),
    }
  }
}

impl<T> Addr<T>
where
  T: 'static + Send + Sync,
{
  /// Create a new address.
  pub fn new() -> (Self, Receiver<T>) {
    let (imp, rx) = DefaultAddrImpl::new();
    (Self { imp: Arc::new(imp) }, rx)
  }

  /// If `U` implements `Into<T>`, we can "upcast" this address
  /// to create an address that can be sent messages of type `U`.
  /// This allows generic message interfaces to be extended by implementors
  /// while remaining compatible with functions that expect the generic
  /// message type.
  pub fn upcast<U>(&self) -> Addr<U>
  where
    U: 'static + Send + Into<T> + Sync,
  {
    Addr {
      imp: Arc::new(CastAddrImpl::new(self.imp.clone())),
    }
  }

  /// Send a message to the actor.
  pub fn send<V: Into<T>>(&self, message: V) -> Result<(), SendError> {
    self.imp.send(message.into())
  }

  /// Create a weak reference to this address.
  pub fn downgrade(&self) -> WeakAddr<T> {
    WeakAddr {
      imp: Arc::downgrade(&self.imp),
    }
  }
}

/// A weak reference to an Actor's address.
pub struct WeakAddr<T>
where
  T: Send + Sync,
{
  imp: Weak<dyn AddrImpl<T> + Send + Sync>,
}

impl<T> Clone for WeakAddr<T>
where
  T: Send + Sync,
{
  fn clone(&self) -> Self {
    Self {
      imp: self.imp.clone(),
    }
  }
}

impl<T> WeakAddr<T>
where
  T: Send + Sync,
{
  pub fn upgrade(&self) -> Option<Addr<T>> {
    self.imp.upgrade().map(|imp| Addr { imp })
  }
}

/// An Actor is a stateful asynchronous "process" that can be
/// sent messages over a channel.
pub trait Actor {
  type Msg: Send + Sync;
  fn spawn(self) -> Addr<Self::Msg>;
}

impl<A> From<A> for Addr<A::Msg>
where
  A: Actor,
{
  fn from(actor: A) -> Self {
    actor.spawn()
  }
}
