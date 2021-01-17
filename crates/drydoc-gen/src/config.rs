use serde::{Serialize, Deserialize};
use super::page::Id;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
  pub name: String,
  pub params: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
  pub id: Id,
  pub name: String,
  pub rule: Rule,
  pub children: Option<Vec<Decl>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Import {
  pub uri: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Decl {
  Unit(Unit),
  Import(Import)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub decl: Decl
}


