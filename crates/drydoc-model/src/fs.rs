use std::collections::HashMap;

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFile {
  content: Box<[u8]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFile {
  path: String
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct LinkedFileHandle(u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedFile {
  handle: LinkedFileHandle
}
#[derive(Serialize, Deserialize, Debug)]
pub enum File {
  Virtual(VirtualFile),
  Local(LocalFile),
  Linked(LinkedFile)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalFolder {
  path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualFolder {
  entries: HashMap<String, Entry>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Folder {
  Virtual(VirtualFolder),
  Local(LocalFolder)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
  File(File),
  Folder(Folder)
}

