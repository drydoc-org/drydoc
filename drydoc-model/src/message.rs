use super::Encoding;

use serde::{Serialize, Deserialize};

use derive_more::*;

#[derive(Display, Debug, Error)]
pub enum DecodeError {
  Invalid
}

#[derive(Serialize, Deserialize)]
pub struct Message(Box<[u8]>);

impl Message {
  pub fn decode(raw: Box<[u8]>) -> Result<Self, DecodeError> {
    let len = raw.len();
    if len < std::mem::size_of::<u32>() + std::mem::size_of::<u8>() {
      return Err(DecodeError::Invalid);
    }

    let ret = Self(raw);
    if ret.size() as usize != len - (std::mem::size_of::<u32>() + std::mem::size_of::<u8>()) {
      return Err(DecodeError::Invalid)
    }

    Ok(ret)
  }

  pub fn encode<T: Serialize>(encoding: Encoding, data: &T) -> Result<Self, Box<dyn std::error::Error>> {
    let data = serde_json::to_vec(data)?;
    let size = (std::mem::size_of::<u8>() + data.len()) as u32;
    let mut encoded = Vec::with_capacity(std::mem::size_of::<u32>() + size as usize);

    encoded.extend(size.to_le_bytes().iter());
    encoded.push(encoding.as_byte());
    encoded.extend(data);
    Ok(Self(encoded.into_boxed_slice()))
  }

  pub fn size(&self) -> u32 {
    let mut size = [0u8; 4];
    size.copy_from_slice(&self.0[0 .. 3]);
    u32::from_le_bytes(size)
  }

  pub fn encoding(&self) -> Option<Encoding> {
    Encoding::from_byte(self.0[0])
  }

  pub fn data(&self) -> &[u8] {
    &self.0[1 ..]
  }
}