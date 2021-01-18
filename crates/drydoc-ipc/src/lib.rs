use std::collections::VecDeque;

use drydoc_model::Message;

use std::mem::size_of;

use tokio::io::{AsyncRead, AsyncWrite};

pub struct MessageProcessor {
  pending: VecDeque<u8>
}

impl MessageProcessor {
  pub fn new() -> Self {
    Self {
      pending: VecDeque::new()
    }
  }
  
  pub fn submit(&mut self, data: &[u8]) {
    self.pending.extend(data);
  }

  pub fn next(&mut self) -> Option<Message> {
    if self.pending.len() < size_of::<u32>() {
      return None;
    }

    let mut size = [0u8; 4];

    let (left, right) = self.pending.as_slices();
    size.copy_from_slice(left);
    if left.len() < size.len() {
      size[left.len() ..].copy_from_slice(right);
    }

    let size = u32::from_le_bytes(size) + size_of::<u8>() as u32;
    if self.pending.len() - size_of::<u32>() < size as usize {
      return None;
    }

    let mut raw = Vec::new();
    let pos = std::cmp::min(left.len(), size as usize);
    raw.extend(&left[0 .. pos]);
    raw.extend(&right[0 .. size as usize - pos]);

    Some(Message::decode(raw.into_boxed_slice()).unwrap())
  }
}

pub struct MessageInputStream<R: AsyncRead> {
  read: R
}

impl<R: AsyncRead> MessageInputStream<R> {
  
}