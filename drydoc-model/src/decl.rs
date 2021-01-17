use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Generate {
  id: String,
  using: String,
  with: HashMap<String, String>,
  children: Option<Vec<Decl>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Import {
  path: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Decl {
  Generate(Generate),
  Import(Import)
}