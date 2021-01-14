use super::actor::{Actor, Addr, Receiver};
use tokio::sync::oneshot::{Sender, channel};

use std::path::Path;

use std::sync::Arc;

use std::collections::HashMap;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use std::path::PathBuf;

use super::resource::{ResourceMsg, GetError};

use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub enum Entry {
  File(Addr<FileMsg>),
  Folder(Addr<FolderMsg>)
}

impl From<Addr<FileMsg>> for Entry {
  fn from(file: Addr<FileMsg>) -> Self {
    Self::File(file)
  }
}

impl From<Addr<FolderMsg>> for Entry {
  fn from(folder: Addr<FolderMsg>) -> Self {
    Self::Folder(folder)
  }
}

impl<T: Actor<Msg = FileMsg>> From<T> for Entry {
  fn from(file: T) -> Self {
    Self::File(file.spawn())
  }
}

impl From<Folder> for Entry {
  fn from(folder: Folder) -> Self {
    Self::Folder(folder.spawn())
  }
}

pub enum FileMsg {
  Resource(ResourceMsg<Arc<[u8]>>),
  WriteTo {
    path: PathBuf,
    sender: Sender<tokio::io::Result<()>>
  }
}

impl From<ResourceMsg<Arc<[u8]>>> for FileMsg {
  fn from(msg: ResourceMsg<Arc<[u8]>>) -> Self {
    Self::Resource(msg)
  }
}

pub struct File {
  file: tokio::fs::File,
}

impl File {
  pub async fn open<P: AsRef<Path>>(path: P) -> tokio::io::Result<Self> {
    let file = tokio::fs::File::open(path).await?;
    
    Ok(Self {
      file
    })
  }

  async fn run(self, mut rx: Receiver<FileMsg>) {
    let mut contents: Option<Arc<[u8]>> = None;
    let mut file = self.file;
    while let Some(msg) = rx.recv().await {
      match msg {
        FileMsg::Resource(msg) => match msg {
          ResourceMsg::Get(sender) => {
            if let Some(contents) = &contents {
              let _ = sender.send(Ok(contents.clone()));
            } else {
              let mut contents_vec = Vec::new();
              match file.read_to_end(&mut contents_vec).await {
                Ok(_) => {
                  let next_contents: Arc<[u8]> = contents_vec.into_boxed_slice().into();
                  contents = Some(next_contents.clone());
                  let _ = sender.send(Ok(next_contents));
                },
                Err(err) => {
                  let _ = sender.send(Err(err.into()));
                }
              }
            }
          }
        },
        FileMsg::WriteTo { path, sender } => {
          if let Some(parent) = path.parent() {
            if let Err(err) = tokio::fs::create_dir_all(parent).await {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
          }

          let mut out_file = match tokio::fs::File::create(path).await {
            Ok(x) => x,
            Err(err) => {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
          };
          
          if let Some(contents) = &contents {
            if let Err(err) = out_file.write_all(contents).await {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
          } else {
            let mut contents_vec = Vec::new();
            if let Err(err) = file.read_to_end(&mut contents_vec).await {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
            let next_contents: Arc<[u8]> = contents_vec.into_boxed_slice().into();
            if let Err(err) = out_file.write_all(&next_contents).await {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
            contents = Some(next_contents);
          }
          
          let _ = sender.send(Ok(()));
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

pub struct VirtFile {
  contents: Arc<[u8]>
}

impl VirtFile {
  pub fn new<C: Into<Arc<[u8]>>>(contents: C) -> Self {
    Self {
      contents: contents.into()
    }
  }

  async fn run(self, mut rx: Receiver<FileMsg>) {
    let contents = self.contents;
    while let Some(msg) = rx.recv().await {
      match msg {
        FileMsg::Resource(msg) => match msg {
          ResourceMsg::Get(sender) => {
            let _ = sender.send(Ok(contents.clone()));
          }
        },
        FileMsg::WriteTo { path, sender } => {
          if let Some(parent) = path.parent() {
            if let Err(err) = tokio::fs::create_dir_all(parent).await {
              sender.send(Err(err)).expect("Send failed");
              continue;
            }
          }

          let mut out_file = match tokio::fs::File::create(path).await {
            Err(err) => {
              sender.send(Err(err)).expect("Send failed");
              continue;
            },
            Ok(x) => x
          };

          if let Err(err) = out_file.write_all(&contents).await {
            sender.send(Err(err)).expect("Send failed");
            continue;
          }

          sender.send(Ok(())).expect("Send failed");
        }
      }
    }
  }
}

impl Actor for VirtFile {
  type Msg = FileMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<FileMsg> {
  pub async fn get(&self) -> Result<Arc<[u8]>, GetError> {
    let (tx, rx) = channel();
    let _ = self.send(ResourceMsg::Get(tx));
    rx.await.unwrap()
  }

  pub async fn write_to<P: AsRef<Path>>(&self, path: P) -> tokio::io::Result<()> {
    let (sender, rx) = channel();
    let _ = self.send(FileMsg::WriteTo { path: path.as_ref().to_path_buf(), sender });
    rx.await.unwrap()
  }
}


pub enum FolderMsg {
  GetEntry {
    name: String,
    sender: Sender<Option<Entry>>
  },
  GetEntries {
    sender: Sender<HashMap<String, Entry>>
  },
  WriteInto {
    path: PathBuf,
    sender: Sender<tokio::io::Result<()>>
  }
}

pub struct Folder {
  entries: HashMap<String, Entry>
}

impl Folder {
  pub fn new() -> Self {
    Self {
      entries: HashMap::new()
    }
  }

  pub fn insert<N: Into<String>, E: Into<Entry>>(&mut self, name: N, entry: E) -> Option<Entry> {

    self.entries.insert(name.into(), entry.into())
  }

  pub fn entries(&self) -> &HashMap<String, Entry> {
    &self.entries
  }

  pub fn entries_mut(&mut self) -> &mut HashMap<String, Entry> {
    &mut self.entries
  }

  pub fn merge(&mut self, folder: &Folder) -> Result<(), ()> {
    for (name, entry) in folder.entries() {
      self.insert(name, entry.clone());
    }
    Ok(())
  }

  pub async fn write_into<P: AsRef<Path>>(self, path: P) -> tokio::io::Result<()> {
    self.spawn().write_into(path).await
  }

  pub fn read<P: 'static + AsRef<Path>>(path: P) -> Pin<Box<dyn Future<Output = tokio::io::Result<Self>>>> {
    Box::pin(async move {
      let mut dir = tokio::fs::read_dir(path).await?;

      let mut entries: HashMap<String, Entry> = HashMap::new();
      while let Ok(Some(entry)) = dir.next_entry().await {
        
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.into_string().unwrap();

        entries.insert(file_name_str, if path.is_dir() {
          Entry::Folder(Self::read(path).await?.spawn())
        } else {
          Entry::File(File::open(path).await?.spawn())
        });
      }

      Ok(Self {
        entries
      })
    })
  }

  async fn run(self, mut rx: Receiver<FolderMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        FolderMsg::GetEntry { name, sender } => {
          let _ = sender.send(self.entries.get(&name).map(|entry| entry.clone()));
        },
        FolderMsg::GetEntries { sender } => {
          let _ = sender.send(self.entries.clone());
        },
        FolderMsg::WriteInto { mut path, sender } => {
          if let Err(err) = tokio::fs::create_dir_all(&path).await {
            sender.send(Err(err)).expect("Send failed");
            continue;
          };

          let mut write_err: Option<tokio::io::Error> = None;

          for (name, entry) in self.entries.iter() {
            path.push(name);
            match entry {
              Entry::File(file) => {
                if let Err(err) = file.write_to(&path).await {
                  println!("File {:?} write err {:?}", &path, &err);
                  write_err = Some(err);
                  break;
                }
              },
              Entry::Folder(folder) => {
                if let Err(err) = folder.write_into(&path).await {
                  println!("Folder {:?} write err {:?}", &path, &err);
                  write_err = Some(err);
                  
                  break;
                }
              }
            }
            path.pop();
          }



          let _ = sender.send(if let Some(err) = write_err {
            Err(err)
          } else {
            Ok(())
          });
        },
        _ => {}
      }
    }
  }
}

impl Actor for Folder {
  type Msg = FolderMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}

impl Addr<FolderMsg> {
  pub async fn get_entry<N: Into<String>>(&self, name: N) -> Option<Entry> {
    let (tx, rx) = channel();
    let _ = self.send(FolderMsg::GetEntry { name: name.into(), sender: tx });
    rx.await.unwrap()
  }

  pub async fn get_entries(&self) -> HashMap<String, Entry> {
    let (tx, rx) = channel();
    let _ = self.send(FolderMsg::GetEntries { sender: tx });
    rx.await.unwrap()
  }

  pub async fn write_into<P: AsRef<Path>>(&self, path: P) -> tokio::io::Result<()> {
    let (tx, rx) = channel();
    let _ = self.send(FolderMsg::WriteInto { path: path.as_ref().to_path_buf(), sender: tx });
    rx.await.unwrap()
  }
}