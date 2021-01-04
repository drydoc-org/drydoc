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
    "h" => "text/h"
  };
}

pub fn lookup<'a, E: AsRef<str>>(extension: E) -> Option<&'static str> {
  CONTENT_TYPE_MAPPINGS.get(extension.as_ref().to_lowercase().as_str()).map(|ty| *ty)
}