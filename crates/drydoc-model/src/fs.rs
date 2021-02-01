//! Serializable filesystem data structures and convenience methods

use std::{
  collections::HashMap,
  io::Write,
  path::{Path, PathBuf},
};

use super::*;

/// An in-memory file.
#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFile {
  content: Box<[u8]>,
}

impl VirtualFile {
  pub fn new<C: Into<Box<[u8]>>>(content: C) -> Self {
    Self {
      content: content.into(),
    }
  }

  pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
    Ok(Self {
      content: std::fs::read(path)?.into_boxed_slice(),
    })
  }

  pub fn content(&self) -> &[u8] {
    &self.content
  }

  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    let mut file = std::fs::File::open(path)?;
    file.write_all(&self.content)?;
    Ok(())
  }

  pub fn content_mut(&mut self) -> &mut [u8] {
    &mut self.content
  }
}

/// A reference to a file on the local filesystem.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFile {
  path: String,
}

impl LocalFile {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      path: path.as_ref().to_string_lossy().to_string(),
    }
  }

  pub fn path(&self) -> &Path {
    self.path.as_ref()
  }

  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    std::fs::copy(&self.path, path).map(|_| ())
  }

  pub fn to_virtual(&self) -> std::io::Result<VirtualFile> {
    VirtualFile::open(&self.path)
  }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct LinkedFileHandle(u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedFile {
  handle: LinkedFileHandle,
}

/// A file
#[derive(Serialize, Deserialize, Debug)]
pub enum File {
  Virtual(VirtualFile),
  Local(LocalFile),
  Linked(LinkedFile),
}

impl File {
  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    match self {
      Self::Virtual(f) => f.write_into(path),
      Self::Local(f) => f.write_into(path),
      Self::Linked(f) => panic!("Can't write a linked file"),
    }
  }
}

impl From<VirtualFile> for File {
  fn from(value: VirtualFile) -> Self {
    Self::Virtual(value)
  }
}

impl From<LocalFile> for File {
  fn from(value: LocalFile) -> Self {
    Self::Local(value)
  }
}

impl From<LinkedFile> for File {
  fn from(value: LinkedFile) -> Self {
    Self::Linked(value)
  }
}

/// A reference to a folder on the local filesystem.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFolder {
  path: String,
}

impl LocalFolder {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      path: path.as_ref().to_string_lossy().to_string(),
    }
  }

  pub fn path(&self) -> &Path {
    self.path.as_ref()
  }

  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    let mut entry_path = path.as_ref().to_path_buf();
    for entry in std::fs::read_dir(&self.path)? {
      let entry = entry?;
      let local_path = entry.path();
      let file_name = entry.file_name();

      entry_path.push(file_name);

      if local_path.is_dir() {
        LocalFolder::new(local_path).write_into(&entry_path)?;
      } else {
        std::fs::copy(local_path, &entry_path)?;
      }

      entry_path.pop();
    }

    Ok(())
  }

  pub fn to_virtual(self) -> std::io::Result<VirtualFolder> {
    let mut ret = VirtualFolder::new();

    for entry in std::fs::read_dir(&self.path)? {
      let entry = entry?;
      let path = entry.path();
      ret.entries.insert(
        entry.file_name().to_str().unwrap().to_string(),
        if path.is_dir() {
          Entry::Folder(LocalFolder::new(&path).into())
        } else {
          Entry::File(LocalFile::new(&path).into())
        },
      );
    }

    Ok(ret)
  }
}

/// An in-memory folder
#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFolder {
  entries: HashMap<String, Entry>,
}

impl VirtualFolder {
  pub fn new() -> Self {
    Self {
      entries: HashMap::new(),
    }
  }

  pub fn insert<N: Into<String>, E: Into<Entry>>(&mut self, name: N, entry: E) -> Option<Entry> {
    self.entries.insert(name.into(), entry.into())
  }

  pub fn get<N: Into<String>>(&mut self, name: N) -> Option<&Entry> {
    self.entries.get(&name.into())
  }

  pub fn get_mut<N: Into<String>>(&mut self, name: N) -> Option<&mut Entry> {
    self.entries.get_mut(&name.into())
  }

  pub fn iter(&self) -> impl Iterator<Item = (&String, &Entry)> {
    self.entries.iter()
  }

  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    std::fs::create_dir_all(&path)?;

    let mut entry_path = PathBuf::new();
    entry_path.push(path);

    for (name, entry) in self.entries.iter() {
      entry_path.push(name);
      entry.write_into(&entry_path)?;
      entry_path.pop();
    }

    Ok(())
  }
}

impl IntoIterator for VirtualFolder {
  type Item = (String, Entry);
  type IntoIter = std::collections::hash_map::IntoIter<String, Entry>;

  fn into_iter(self) -> Self::IntoIter {
    self.entries.into_iter()
  }
}

/// A folder
#[derive(Serialize, Deserialize, Debug)]
pub enum Folder {
  Virtual(VirtualFolder),
  Local(LocalFolder),
}

impl Folder {
  /// Merge the contents of the folder `other` into this one,
  /// returning the resulting folder.
  pub fn merge<O: Into<Self>>(self, other: O) -> std::io::Result<Self> {
    let other = other.into().to_virtual()?;
    let mut this = self.to_virtual()?;
    this.entries.extend(other.entries);
    Ok(this.into())
  }

  /// Writes the folder into the directory `path`.
  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    match self {
      Self::Virtual(f) => f.write_into(path),
      Self::Local(f) => f.write_into(path),
    }
  }

  /// Convert this folder to a `VirtualFolder`.
  pub fn to_virtual(self) -> std::io::Result<VirtualFolder> {
    match self {
      Self::Local(local) => local.to_virtual(),
      Self::Virtual(virt) => Ok(virt),
    }
  }
}

impl From<VirtualFolder> for Folder {
  fn from(value: VirtualFolder) -> Self {
    Self::Virtual(value)
  }
}

impl From<LocalFolder> for Folder {
  fn from(value: LocalFolder) -> Self {
    Self::Local(value)
  }
}

/// An entry in a folder (can be either a child file or folder)
#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
  File(File),
  Folder(Folder),
}

impl Entry {
  pub fn write_into<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    match self {
      Self::File(f) => f.write_into(path),
      Self::Folder(f) => f.write_into(path),
    }
  }

  pub fn as_file(&self) -> Option<&File> {
    if let Self::File(file) = self {
      Some(file)
    } else {
      None
    }
  }
}

impl From<File> for Entry {
  fn from(value: File) -> Self {
    Self::File(value)
  }
}

impl From<VirtualFile> for Entry {
  fn from(value: VirtualFile) -> Self {
    Self::File(File::Virtual(value))
  }
}

impl From<LocalFile> for Entry {
  fn from(value: LocalFile) -> Self {
    Self::File(File::Local(value))
  }
}

impl From<VirtualFolder> for Entry {
  fn from(value: VirtualFolder) -> Self {
    Self::Folder(Folder::Virtual(value))
  }
}

impl From<LocalFolder> for Entry {
  fn from(value: LocalFolder) -> Self {
    Self::Folder(Folder::Local(value))
  }
}

impl From<Folder> for Entry {
  fn from(value: Folder) -> Self {
    Self::Folder(value)
  }
}
