use std::{fmt::{Display, Formatter, Result}, sync::Arc};

pub struct Namespace {
  parent: Option<Arc<Namespace>>,
  name: String
}

impl Namespace {
  pub fn new<N: Into<String>>(name: N) -> Arc<Self> {
    Arc::new(Self {
      parent: None,
      name: name.into()
    })
  }

  pub fn child<N: Into<String>>(self: Arc<Self>, name: N) -> Arc<Self> {
    Arc::new(Self {
      parent: Some(self),
      name: name.into()
    })
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    if let Some(parent) = &self.parent {
      write!(f, "{}/{}", parent, self.name)
    } else {
      write!(f, "{}", self.name)
    }
  }
}