use std::{collections::HashMap, hash::Hash};

use super::{Actor, Addr, Receiver, SendError};

use tokio::sync::oneshot::{channel, Sender};

pub struct Insert<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  pub key: K,
  pub value: V,
  pub res: Sender<Option<V>>,
}

pub struct Remove<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  pub key: K,
  pub res: Sender<Option<V>>,
}

pub enum Msg<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  Insert(Insert<K, V>),
  Remove(Remove<K, V>),
}

impl<K, V> From<Insert<K, V>> for Msg<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  fn from(value: Insert<K, V>) -> Self {
    Self::Insert(value)
  }
}

impl<K, V> From<Remove<K, V>> for Msg<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  fn from(value: Remove<K, V>) -> Self {
    Self::Remove(value)
  }
}

/// An actor-based implementation of a HashMap.
pub struct Store<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  entries: HashMap<K, V>,
}

impl<K, V> Store<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  pub fn new() -> Self {
    Self {
      entries: HashMap::new(),
    }
  }

  async fn run(mut self, mut rx: Receiver<Msg<K, V>>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        Msg::Insert(Insert { key, value, res }) => {
          let _ = res.send(self.entries.insert(key, value));
        }
        Msg::Remove(Remove { key, res }) => {
          let _ = res.send(self.entries.remove(&key));
        }
      }
    }
  }
}

impl<K, V> Actor for Store<K, V>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  type Msg = Msg<K, V>;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl<K, V> Addr<Msg<K, V>>
where
  K: 'static + Send + Sync + Eq + Hash,
  V: 'static + Send + Sync,
{
  pub async fn insert(&self, key: K, value: V) -> Result<Option<V>, SendError> {
    let (tx, rx) = channel();
    self.send(Insert {
      key,
      value,
      res: tx,
    })?;
    Ok(rx.await.unwrap())
  }

  pub async fn remove(&self, key: K) -> Result<Option<V>, SendError> {
    let (tx, rx) = channel();
    self.send(Remove { key, res: tx })?;
    Ok(rx.await.unwrap())
  }
}
