use super::Emitter;

use drydoc_model::bundle::Bundle;
use std::{io::Result, path::{Path, PathBuf}};

use std::iter::FromIterator;

use crate::fs2::{Folder, RealFile, VirtFile};

pub struct Html {
  dir: PathBuf,
}

impl Html {
  pub fn new<P: AsRef<Path>>(dir: P) -> Self {
    Self {
      dir: dir.as_ref().to_path_buf()
    }
  }
}



#[async_trait::async_trait]
impl Emitter for Html {
  async fn emit(&self, mut bundle: Bundle) -> Result<()> {
    let manifest_js = format!("window.MANIFEST = {}", serde_json::to_string_pretty(&bundle.manifest).unwrap());

    let current_exe = std::env::current_exe().unwrap();
    let home = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();

    let mut js = Folder::new();
    let bundle_path = home.join(PathBuf::from_iter(&["client", "dist", "bundle.js"]));

    js.insert("bundle.js", RealFile::open(bundle_path)?)?;
    js.insert("manifest.js", VirtFile::new(manifest_js.as_bytes()))?;

    bundle.insert_entry("js", js)?;
    bundle.folder.merge(Folder::read(home.join("static")).await?).unwrap();
    bundle.write_out(self.dir.clone()).await?;

    Ok(())
  }

}

