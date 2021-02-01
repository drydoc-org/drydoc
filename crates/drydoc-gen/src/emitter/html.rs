use super::Emitter;

use bytes::BytesMut;
use drydoc_model::{
  bundle::Bundle,
  fs::{LocalFile, LocalFolder, VirtualFile, VirtualFolder},
};
use std::{
  io::{Result, Write},
  path::{Path, PathBuf},
};

use std::iter::FromIterator;

pub struct Html {
  dir: PathBuf,
}

impl Html {
  pub fn new<P: AsRef<Path>>(dir: P) -> Self {
    Self {
      dir: dir.as_ref().to_path_buf(),
    }
  }
}

#[async_trait::async_trait]
impl Emitter for Html {
  async fn emit(&self, mut bundle: Bundle) -> Result<()> {
    let mut encoder = compress::lz4::Encoder::new(Vec::new());
    encoder.write(serde_json::to_vec(&bundle.manifest)?.as_slice())?;
    let (compressed_manifest, res) = encoder.finish();
    res?;

    let manifest_js = format!(
      "window.MANIFEST = \"{}\";",
      base64::encode(compressed_manifest)
    );

    let current_exe = std::env::current_exe().unwrap();
    let home = current_exe
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .parent()
      .unwrap();

    let mut js_folder = VirtualFolder::new();
    js_folder.insert(
      "bundle.js",
      LocalFile::new(home.join(PathBuf::from_iter(&["client", "dist", "bundle.js"]))),
    );
    js_folder.insert("manifest.js", VirtualFile::new(manifest_js.as_bytes()));

    bundle = bundle.insert_entry("js", js_folder)?;
    bundle.resources = bundle
      .resources
      .merge(LocalFolder::new(home.join("static")))?;

    bundle.resources.write_into(self.dir.clone())?;

    Ok(())
  }
}
