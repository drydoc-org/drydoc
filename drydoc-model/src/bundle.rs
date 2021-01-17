use std::collections::HashMap;

use super::*;

use page::{Id, Page};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
  root: Id,
  pages: HashMap<Id, Page>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
  manifest: Manifest,
  resources: fs::Folder
}