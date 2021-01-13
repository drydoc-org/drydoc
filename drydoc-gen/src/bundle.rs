use serde::{Serialize, Deserialize};

use super::page::{Page, Id};
use std::{collections::HashMap, path::{Path, PathBuf}};
use tokio::fs;
use super::actor::{Actor, Addr};
use super::fs::{FolderMsg, Folder, VirtFile, Entry};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
  pub root: Id,
  pub pages: HashMap<Id, Page>
}

impl Manifest {
  pub fn new<R: Into<Id>>(root: R, pages: HashMap<Id, Page>) -> Self {
    Self {
      root: root.into(),
      pages
    }
  }
  
  pub fn merge(&mut self, child: Manifest) -> Result<(), ()> {
    if let Some(page) = self.pages.get_mut(&self.root) {
      page.children.insert(child.root);
    }
    
    for (id, page) in child.pages {
      self.pages.insert(id, page);
    }
    Ok(())
  }

  pub fn namespace(mut self) -> Self {
    let mut pages = HashMap::new();
    
    let mut q: Vec<(String, Id)> = vec![ ("".to_string(), self.root.clone()) ];
    while let Some((prefix, current)) = q.pop() {
      let page = self.pages.remove(&current).unwrap();
      let next_id = Id(format!("{}{}", prefix, current.0));
      for child in page.children.iter() {
        q.push((next_id.0.clone(), child.clone()))
      }
      let next_children = page.children.into_iter().map(|c| Id(format!("{}{}", next_id.0, c.0))).collect();

      
      
      let next_page = Page {
        id: next_id.clone(),
        children: next_children,
        ..page
      };

      

      pages.insert(next_id.clone(), next_page);
    }

    Self {
      root: self.root,
      pages
    }
  }
}

pub struct Bundle {
  pub manifest: Manifest,
  pub folder: Folder
}

impl Bundle {
  pub fn new(manifest: Manifest) -> Self {
    Self {
      manifest,
      folder: Folder::new()
    }
  }

  pub fn insert_entry<N: Into<String>, E: Into<Entry>>(&mut self, name: N, entry: E) -> Option<Entry> {
    self.folder.insert(name, entry)
  }

  pub fn merge(&mut self, child: Bundle) -> Result<(), ()> {

    self.manifest.merge(child.manifest)?;
    self.folder.merge(&child.folder)?;
    Ok(())
  }

  pub async fn write_out<'a, P: 'a + AsRef<Path>>(&self, path: P) -> tokio::io::Result<()> {
    let mut folder = Folder::new();
    let manifest_json = serde_json::to_vec(&self.manifest).unwrap().into_boxed_slice();
    folder.insert("manifest.json", VirtFile::new(manifest_json));
    folder.merge(&self.folder).unwrap();
    folder.write_into(path).await
  }
  
  pub fn namespace(self) -> Self {
    Self {
      manifest: self.manifest.namespace(),
      folder: self.folder
    }
  }
}
