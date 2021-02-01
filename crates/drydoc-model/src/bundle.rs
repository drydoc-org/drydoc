use std::collections::HashMap;

use super::*;

use page::{Id, Page};

use crate::fs::Entry;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
  pub root: Id,
  pub symbols: HashMap<String, Vec<Id>>,
  pub pages: HashMap<Id, Page>,
}

impl Manifest {
  pub fn merge(&mut self, other: Manifest) {
    self.symbols.extend(other.symbols);
    self.pages.extend(other.pages);
    if let Some(root) = self.pages.get_mut(&self.root) {
      root.children.insert(other.root);
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
  pub manifest: Manifest,
  pub resources: fs::Folder,
}

impl Bundle {
  pub fn insert_entry<N: Into<String>, E: Into<Entry>>(
    mut self,
    name: N,
    entry: E,
  ) -> std::io::Result<Bundle> {
    let mut virt_resources = self.resources.to_virtual()?;
    virt_resources.insert(name, entry);
    self.resources = virt_resources.into();
    Ok(self)
  }

  pub fn merge(mut self, other: Bundle) -> std::io::Result<Bundle> {
    self.manifest.merge(other.manifest);
    self.resources = self.resources.merge(other.resources)?;
    Ok(self)
  }
}
