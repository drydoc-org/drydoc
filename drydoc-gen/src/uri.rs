pub use url::Url as Uri;

pub fn to_uri(uri_like: &str) -> Uri {
  let path = std::fs::canonicalize(uri_like).unwrap();
  let mut path_str = path.as_path().to_str().unwrap().to_string();
  if cfg!(windows) {
    path_str = path_str.split_off(4);
    path_str = path_str.replace('\\', "/");
  }

  let uri_str = format!("file://{}", path_str);
  Uri::parse(uri_str.as_str()).unwrap()
}