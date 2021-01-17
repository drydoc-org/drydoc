use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc};

use std::io::{ErrorKind, Error, Result, Read};

use tokio::io::AsyncWriteExt;

use memmap::{Mmap, MmapOptions};

use std::pin::Pin;
use std::future::Future;

pub enum Entry {
  File(Arc<dyn File + Send + Sync>),
  Folder(Folder)
}

impl Entry {
  pub fn as_file(&self) -> Option<&Arc<dyn File + Send + Sync>> {
    if let Self::File(file) = self {
      Some(file)
    } else {
      None
    }
  }

  pub fn as_folder(&self) -> Option<&Folder> {
    if let Self::Folder(folder) = self {
      Some(folder)
    } else {
      None
    }
  }

  pub fn as_folder_mut(&mut self) -> Option<&mut Folder> {
    if let Self::Folder(folder) = self {
      Some(folder)
    } else {
      None
    }
  }
}

impl From<Arc<dyn File + Send + Sync>> for Entry {
  fn from(value: Arc<dyn File + Send + Sync>) -> Self {
    Self::File(value)
  }
}

impl<T: 'static + File + Send + Sync> From<T> for Entry {
  fn from(value: T) -> Self {
    Self::File(Arc::new(value))
  }
}

impl From<Folder> for Entry {
  fn from(value: Folder) -> Self {
    Self::Folder(value)
  }
}

#[async_trait::async_trait]
pub trait File {
  fn contents(&self) -> &[u8];
  async fn write(&self, path: &PathBuf) -> Result<()>;
}

pub struct VirtFile {
  contents: Box<[u8]>,
}

impl VirtFile {
  pub fn new<C: Into<Box<[u8]>>>(contents: C) -> Self {
    Self {
      contents: contents.into()
    }
  }
}

pub struct RealFile {
  file: std::fs::File,
  contents: Option<Mmap>,
}

impl RealFile {
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file = std::fs::File::open(path)?;
    
    

    Ok(Self {
      contents: if file.metadata()?.len() > 0 { Some(unsafe { Mmap::map(&file)? }) } else { None },
      file,
    })
  }
}

static EMPTY: &'static [u8] = &[];

#[async_trait::async_trait]
impl File for RealFile {
  fn contents(&self) -> &[u8] {
    if let Some(contents) = &self.contents {
      contents
    } else {
      EMPTY
    }
  }

  async fn write(&self, path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
      tokio::fs::create_dir_all(parent).await?;
    }

    let mut out_file = tokio::fs::File::create(path).await?;
    out_file.write_all(self.contents()).await?;
    Ok(())
  }
}

#[async_trait::async_trait]
impl File for VirtFile {
  fn contents(&self) -> &[u8] {
    &self.contents
  }

  async fn write(&self, path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
      tokio::fs::create_dir_all(parent).await?;
    }

    let mut out_file = tokio::fs::File::create(path).await?;
    out_file.write_all(&self.contents).await?;
    Ok(())
  }
}

pub struct Folder {
  entries: HashMap<String, Entry>,
}

impl Folder {
  pub fn new() -> Self {
    Self {
      entries: HashMap::new()
    }
  }

  pub fn read(path: PathBuf) -> Pin<Box<dyn Future<Output = Result<Self>> + Send>> {
    Box::pin(async move {
      let mut dir = tokio::fs::read_dir(path).await?;

      let mut entries: HashMap<String, Entry> = HashMap::new();
      while let Ok(Some(entry)) = dir.next_entry().await {
        
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.into_string().unwrap();

        entries.insert(file_name_str, if path.is_dir() {
          Self::read(path).await?.into()
        } else {
          RealFile::open(path)?.into()
        });
      }

      Ok(Self {
        entries
      })
    })
  }

  pub fn insert_path<N: AsRef<str>, E: Into<Entry>>(&mut self, path: &[N], entry: E) -> Result<()> {
    match path.split_first() {
      Some((first_component, rest)) => {
        let first_component_string = first_component.as_ref().to_string();
        if rest.is_empty() {
          if let Some(_) = self.entries.get(&first_component_string) {
            Err(Error::new(ErrorKind::AlreadyExists, format!("{} already exists", &first_component_string)))
          } else {
            self.entries.insert(first_component_string, entry.into());
            Ok(())
          }
        } else {
          if let Some(inner_entry) = self.entries.get_mut(&first_component_string) {
            if let Entry::Folder(folder) = inner_entry {
              folder.insert_path(rest, entry)
            } else {
              Err(Error::new(ErrorKind::AlreadyExists, format!("{} is a folder", first_component.as_ref())))
            }
          } else {
            let mut folder = Folder::new();
            folder.insert_path(rest, entry)?;
            self.entries.insert(first_component_string, folder.into());
            Ok(())
          }
        }
      },
      None => {
        Err(Error::new(ErrorKind::InvalidInput, "Path is empty"))
      }
    }
  }

  pub fn insert<N: Into<String>, E: Into<Entry>>(&mut self, name: N, entry: E) -> Result<()> {
    let name: String = name.into();
    let entry: Entry = entry.into();
    let parts: Vec<&str> = name.split('/').collect();
    self.insert_path(parts.as_slice(), entry)
  }

  pub fn merge(&mut self, other: Folder) -> Result<()> {
    for (name, entry) in other.entries {
      if let Some(self_entry) = self.entries.get_mut(&name) {
        if let Entry::Folder(self_entry) = self_entry {
          if let Entry::Folder(entry) = entry {
            self_entry.merge(entry)?;
          } else {
            return Err(Error::new(ErrorKind::AlreadyExists,format!("Expected {} to be a folder", &name)))
          }
        } else {
          return Err(Error::new(ErrorKind::AlreadyExists,format!("Expected {} to be a folder", &name)))
        }
      } else {
        self.entries.insert(name, entry);
        
      }
    }

    Ok(())
  }

  pub fn write_into(&self, mut path: PathBuf) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
    Box::pin(async move {
      tokio::fs::create_dir_all(&path).await?;

      for (name, entry) in self.entries.iter() {
        path.push(name);
        match entry {
          Entry::File(file) => {
            file.write(&path).await?;
          },
          Entry::Folder(folder) => {
            folder.write_into(path.clone()).await?;
          }
        }
        path.pop();
      }

      Ok(())
    })
  }
}