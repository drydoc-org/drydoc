use std::collections::HashMap;

use super::*;

use page::{Id, Page};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
  root: Id,
  symbols: HashMap<String, Vec<Id>>,
  pages: HashMap<Id, Page>
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
  manifest: Manifest,
  resources: fs::Folder
}

impl Bundle {
  pub fn merge(&mut self, other: Bundle) {
    self.manifest.merge(other.manifest);
  }
}