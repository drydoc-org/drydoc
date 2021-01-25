use std::io::Result;

use drydoc_model::bundle::Bundle;

pub mod html;

#[async_trait::async_trait]
pub trait Emitter {
  async fn emit(&self, bundle: Bundle) -> Result<()>;
}