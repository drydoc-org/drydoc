use serde_yaml::{Value, Sequence, Mapping};

use tokio::io::Result;

use std::pin::Pin;
use std::future::Future;

use std::io;
use io::ErrorKind;
use tokio::sync::oneshot::{Receiver, channel};

use std::process::{Command, Stdio};

use std::path::{Path, PathBuf};

use std::sync::Arc;

lazy_static! {
  pub static ref CMD_REGEX: regex::Regex = regex::Regex::new(r"\$\(.*\)").unwrap();
}


fn which(name: &str) -> Result<PathBuf> {
  let sys_path = std::env::var("PATH").unwrap();
  let sys_paths: Vec<&str> = sys_path.split(|c| c == if cfg!(windows) { ';' } else { ':' }).collect();

  
  for sys_path in sys_paths {
    for entry in std::fs::read_dir(sys_path)? {
      let entry = entry?;
      let path = entry.path();
      let mut file_name = path.file_stem().unwrap().to_str().unwrap().to_string();

      if file_name == name {
        return Ok(path.to_path_buf())
      }
    }
  }

  Err(io::Error::new(ErrorKind::NotFound, format!("{} not found in PATH", name)))
}

// FIXME: This should be async, but the regex replace_all function isn't.
// Write an async-compat replace_all.
fn execute(cmd: &str, working_dir: Arc<PathBuf>) -> Result<String> {
  let args: Vec<&str> = cmd.split_ascii_whitespace().collect();

  let (name, rest) = match args.split_first() {
    None => {
      return Err(io::Error::new(ErrorKind::InvalidInput, "Empty command"));
    },
    Some(x) => x
  };

  let bin = which(name)?;

  let cmd = Command::new(bin)
    .args(rest)
    .stdout(Stdio::piped())
    .current_dir(working_dir.as_ref())
    .spawn()?;
  
  let out = cmd.wait_with_output()?;

  let ret = String::from_utf8(out.stdout).unwrap();
  Ok(ret.trim().to_string())
}

pub fn preprocess(yaml: Value, working_dir: Arc<PathBuf>) -> Pin<Box<dyn Future<Output = Result<Value>>>> {
  Box::pin(async move {
    match yaml {
      Value::Mapping(map) => {
        let mut next = Mapping::new();

        for (key, value) in map.into_iter() {
          let key = preprocess(key, working_dir.clone()).await?;
          let value = preprocess(value, working_dir.clone()).await?;

          next.insert(key, value);
        }

        Ok(Value::Mapping(next))
      },
      Value::String(string) => {
        Ok(Value::String(CMD_REGEX.replace_all(string.as_str(), |cap: &regex::Captures| {
          let cap = &cap[0];
          let cap = &cap[2 .. cap.len() - 1];
          execute(cap, working_dir.clone()).unwrap()
        }).to_string()))
      },
      Value::Sequence(seq) => {
        let mut next = Sequence::new();
        for value in seq {
          next.push(preprocess(value, working_dir.clone()).await?);
        }
        Ok(Value::Sequence(next))
      },
      x => Ok(x.clone())
    }
  })
}