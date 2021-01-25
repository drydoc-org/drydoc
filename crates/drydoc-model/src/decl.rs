use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Generate {
  pub id: String,
  pub using: String,
  pub with: HashMap<String, String>,
  pub children: Option<Vec<Decl>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Import {
  pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Decl {
  Generate(Generate),
  Import(Import)
}