use std::pin::Pin;
use std::future::Future;
use std::path::{Path, PathBuf};

pub fn get_files<P: Into<PathBuf>>(path: P, pred: fn(&Path) -> bool) -> Pin<Box<dyn Future<Output = tokio::io::Result<Vec<PathBuf>>>>> {
  let path = path.into();
  Box::pin(async move {
    if path.is_dir() {
      let mut dir = tokio::fs::read_dir(path).await?;

      let mut ret = Vec::new();
      while let Ok(Some(entry)) = dir.next_entry().await {
        let subpaths = get_files(entry.path(), pred).await?;
        ret.extend_from_slice(subpaths.as_slice());
      }

      Ok(ret)
    } else {
      Ok(vec![ path.to_path_buf() ])
    }
  })
}