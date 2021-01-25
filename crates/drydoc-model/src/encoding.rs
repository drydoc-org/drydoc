use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Encoding {
  Json,
  Pickle,
  Bincode,
}

impl Encoding {
  pub fn as_byte(&self) -> u8 {
    match self {
      Self::Json => 0,
      Self::Pickle => 1,
      Self::Bincode => 2
    }
  }

  pub fn from_byte(value: u8) -> Option<Self> {
    match value {
      0 => Some(Self::Json),
      1 => Some(Self::Pickle),
      2 => Some(Self::Bincode),
      _ => None
    }
  }
}
