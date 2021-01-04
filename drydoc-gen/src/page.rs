use serde::{Serialize, Deserialize};

use std::fmt::{Display, Formatter};

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Id(pub String);

impl From<String> for Id {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl<'a> From<&'a str> for Id {
  fn from(value: &'a str) -> Self {
    Self(value.to_string())
  }
}

impl From<&String> for Id {
  fn from(value: &String) -> Self {
    Self(value.clone())
  }
}

impl Display for Id {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0.as_str())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
  pub id: Id,
  pub name: String,
  pub renderer: String,
  pub content_type: String,
  pub metadata: HashMap<String, String>,
  pub children: HashSet<Id>
}

impl Page {
  pub fn builder() -> PageBuilder {
    PageBuilder::new()
  }
}

pub struct PageBuilder {
  id: Option<Id>,
  name: Option<String>,
  renderer: Option<String>,
  content_type: Option<String>,
  metadata: HashMap<String, String>,
  children: HashSet<Id>
}

#[derive(Debug)]
pub enum BuildError {
  MissingId,
  MissingName,
  MissingRenderer,
  MissingContentType
}

impl PageBuilder {
  pub fn new() -> Self {
    Self {
      id: None,
      name: None,
      renderer: None,
      content_type: None,
      metadata: HashMap::new(),
      children: HashSet::new()
    }
  }

  pub fn id<I: Into<Id>>(mut self, id: I) -> Self {
    self.id = Some(id.into());
    self
  }

  pub fn name<N: Into<String>>(mut self, name: N) -> Self {
    self.name = Some(name.into());
    self
  }

  pub fn renderer<R: Into<String>>(mut self, renderer: R) -> Self {
    self.renderer = Some(renderer.into());
    self
  }

  pub fn content_type<C: Into<String>>(mut self, content_type: C) -> Self {
    self.content_type = Some(content_type.into());
    self
  }

  pub fn child<C: Into<Id>>(mut self, child: C) -> Self {
    self.children.insert(child.into());
    self
  }

  pub fn children<T: Into<Id>, I: Iterator<Item = T>>(mut self, iter: I) -> Self {
    self.children.extend(iter.map(|i| i.into()));
    self
  }

  pub fn meta<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.metadata.insert(key.into(), value.into());
    self
  }
  
  pub fn build(mut self) -> Result<Page, BuildError> {
    let id = match self.id.take() {
      Some(id) => id,
      None => return Err(BuildError::MissingId)
    };

    let name = match self.name.take() {
      Some(name) => name,
      None => return Err(BuildError::MissingName)
    };

    let renderer = match self.renderer.take() {
      Some(renderer) => renderer,
      None => return Err(BuildError::MissingRenderer)
    };

    let content_type = match self.content_type.take() {
      Some(content_type) => content_type,
      None => return Err(BuildError::MissingContentType)
    };
    
    Ok(Page {
      id,
      name,
      renderer,
      content_type,
      metadata: self.metadata,
      children: self.children
    })
  }
}

