use crate::actor::{Actor, Addr, Receiver};

use tokio::sync::oneshot::{channel, Sender};

use std::collections::{HashMap, HashSet};

pub enum ProgressMsg {
  StartTask {
    parent: Option<u64>,
    name: String,
    details: Option<String>,
    sender: Sender<Result<u64, ()>>,
  },
  FinishTask {
    id: u64,
  },
}

struct Task {
  parent: Option<u64>,
  name: String,
  details: Option<String>,
  children: HashSet<u64>,
}

impl Task {
  pub fn root(name: String, details: Option<String>) -> Self {
    Self {
      name,
      details,
      parent: None,
      children: HashSet::new(),
    }
  }

  pub fn child(parent: u64, name: String, details: Option<String>) -> Self {
    Self {
      name,
      details,
      parent: Some(parent),
      children: HashSet::new(),
    }
  }
}

pub struct Progress {
  roots: HashSet<u64>,
  tasks: HashMap<u64, Task>,
  task_iter: u64,
}

impl Progress {
  pub fn new() -> Self {
    Self {
      roots: HashSet::new(),
      tasks: HashMap::new(),
      task_iter: 0,
    }
  }

  fn update(&self) {}

  async fn run(mut self, mut rx: Receiver<ProgressMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        ProgressMsg::StartTask {
          parent,
          name,
          details,
          sender,
        } => {
          let id = self.task_iter;
          self.task_iter += 1;

          if let Some(parent) = &parent {
            match self.tasks.get_mut(parent) {
              Some(parent) => {
                parent.children.insert(id);
              }
              None => {
                let _ = sender.send(Err(()));
              }
            }
          } else {
            self.roots.insert(id);
          }

          self.tasks.insert(
            id,
            Task {
              parent,
              name,
              details,
              children: HashSet::new(),
            },
          );
        }
        ProgressMsg::FinishTask { id } => {
          let task = match self.tasks.remove(&id) {
            Some(task) => task,
            None => continue,
          };

          if let Some(parent) = &task.parent {
            if let Some(parent) = self.tasks.get_mut(parent) {
              parent.children.remove(&id);
            }
          } else {
            self.roots.remove(&id);
          }
        }
      }
    }
  }
}

impl Actor for Progress {
  type Msg = ProgressMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<ProgressMsg> {
  pub async fn start_task(
    &self,
    parent: Option<u64>,
    name: String,
    details: Option<String>,
  ) -> Result<u64, ()> {
    let (tx, rx) = channel();
    let _ = self.send(ProgressMsg::StartTask {
      parent,
      name,
      details,
      sender: tx,
    });
    rx.await.unwrap()
  }

  pub fn finish_task(&self, id: u64) {
    let _ = self.send(ProgressMsg::FinishTask { id });
  }
}
