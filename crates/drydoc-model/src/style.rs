use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Style(HashMap<String, String>);

impl Style {
  pub fn get<K: Into<String>>(&self, key: K) -> Option<&String> {
    self.0.get(&key.into())
  }
}
