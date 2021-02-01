use std::{
  fmt::{Display, Formatter},
  sync::Arc,
};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use derive_more::{Display, Error};
#[derive(Display, Debug, Error)]
pub enum ParseError {
  EmptyComponent,
}

pub struct Namespace {
  parent: Option<Arc<Namespace>>,
  name: String,
}

impl Namespace {
  pub fn new<N: Into<String>>(name: N) -> Arc<Self> {
    let name: String = name.into();
    assert!(!name.contains('.'));

    Arc::new(Self { parent: None, name })
  }

  pub fn child<N: Into<String>>(self: &Arc<Self>, name: N) -> Arc<Self> {
    let name: String = name.into();
    assert!(!name.contains('.'));

    Arc::new(Self {
      parent: Some(self.clone()),
      name,
    })
  }

  pub fn from_str<S: AsRef<str>>(str: S) -> Result<Self, ParseError> {
    let str = str.as_ref();
    if let Some(last_start) = str.rfind('.') {
      Ok(Self {
        parent: Some(Arc::new(Self::from_str(&str[..last_start])?)),
        name: str[last_start + 1..].to_string(),
      })
    } else {
      Ok(Self {
        parent: None,
        name: str.to_string(),
      })
    }
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if let Some(parent) = &self.parent {
      write!(f, "{}/{}", parent, self.name)
    } else {
      write!(f, "{}", self.name)
    }
  }
}

impl Serialize for Namespace {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

// impl<'de> Deserialize<'de> for Namespace {
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//   where
//     D: Deserializer<'de>
//   {
//     struct StrVisitor {

//     }

//     impl Visitor for StrVisitor {

//     }

//     deserializer.deserialize_str();
//   }
// }
