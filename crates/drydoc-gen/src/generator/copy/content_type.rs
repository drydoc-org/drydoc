use std::collections::HashMap;

macro_rules! map(
  { $($key:expr => $value:expr),+ } => {
      {
          let mut m = ::std::collections::HashMap::new();
          $(
              m.insert($key, $value);
          )+
          m
      }
   };
);

lazy_static! {
  pub static ref CONTENT_TYPE_MAPPINGS: HashMap<&'static str, &'static str> = map! {
    "md" => "text/markdown",
    "markdown" => "text/markdown",
    "cpp" => "text/cpp",
    "cxx" => "text/cpp",
    "c" => "text/c",
    "hpp" => "text/hpp",
    "hxx" => "text/hpp",
    "h" => "text/h",
    "webm" => "video/webm",
    "mp4" => "video/mp4",
    "gif" => "image/gif",
    "jpg" => "image/jpeg",
    "jpeg" => "image/jpeg",
    "svg" => "image/svg+xml",
    "webp" => "image/webp",
    "apng" => "image/apng",
    "avif" => "image/avif",
    "bmp" => "image/bmp",
    "ico" => "image/x-icon",
    "tiff" => "image/tiff",
    "flac" => "audio/flac",
    "ogg" => "audio/ogg",
    "mov" => "video/quicktime",
    "wav" => "audio/wav"
  };
}

pub fn lookup<'a, E: AsRef<str>>(extension: E) -> Option<&'static str> {
  CONTENT_TYPE_MAPPINGS.get(extension.as_ref().to_lowercase().as_str()).map(|ty| *ty)
}