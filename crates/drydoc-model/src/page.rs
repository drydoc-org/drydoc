use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter};

use std::collections::HashMap;
use std::collections::HashSet;

use derive_more::*;

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
  pub content_type: String,
  pub metadata: HashMap<String, String>,
  pub children: HashSet<Id>,
  pub url: Option<String>,
  pub hidden: Option<bool>,
}

impl Page {
  pub fn builder() -> PageBuilder {
    PageBuilder::new()
  }
}

pub struct PageBuilder {
  id: Option<Id>,
  name: Option<String>,
  content_type: Option<String>,
  metadata: HashMap<String, String>,
  children: HashSet<Id>,
  url: Option<String>,
  hidden: Option<bool>,
}

#[derive(Display, Debug, Error)]
pub enum BuildError {
  MissingId,
  MissingName,
  MissingContentType,
}

impl PageBuilder {
  pub fn new() -> Self {
    Self {
      id: None,
      name: None,
      content_type: None,
      url: None,
      hidden: None,
      metadata: HashMap::new(),
      children: HashSet::new(),
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

  pub fn url<U: Into<String>>(mut self, url: U) -> Self {
    self.url = Some(url.into());
    self
  }

  pub fn hidden<H: Into<bool>>(mut self, hidden: H) -> Self {
    self.hidden = Some(hidden.into());
    self
  }

  pub fn build(mut self) -> Result<Page, BuildError> {
    let id = match self.id.take() {
      Some(id) => id,
      None => return Err(BuildError::MissingId),
    };

    let name = match self.name.take() {
      Some(name) => name,
      None => return Err(BuildError::MissingName),
    };

    let content_type = match self.content_type.take() {
      Some(content_type) => content_type,
      None => return Err(BuildError::MissingContentType),
    };

    Ok(Page {
      id,
      name,
      content_type,
      url: self.url,
      hidden: self.hidden,
      metadata: self.metadata,
      children: self.children,
    })
  }
}
