use url::Url;

use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

pub async fn fetch(uri: &Url) -> Result<Box<[u8]>, tokio::io::Error> {
  match uri.scheme() {
    "" | "file" => {
      let mut path_str = uri.path().to_string();
      if cfg!(windows) {
        path_str = path_str.replace('/', "\\").split_off(1);
      }
      let mut file = tokio::fs::File::open(path_str.as_str()).await?;
      let mut contents = Vec::new();
      file.read_to_end(&mut contents).await?;
      Ok(contents.into_boxed_slice())
    },
    _ => Err(tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, "URI scheme not recognized"))
  }
}