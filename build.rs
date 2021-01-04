use std::{fs::{DirEntry, FileType}, path::{Path, PathBuf}, process::Command};

struct CommandRes {
  exit_code: i32,
  stdout: String,
  stderr: String
}

fn find_program(name: &str) -> Option<PathBuf> {
  let path = std::env::var("PATH").expect("Unable to retrieve PATH environment variable");
  let components = path.split(|c| {
    if cfg!(windows) { c == ';' } else { c == ':' }
  });

  for component in components {
    if component.len() == 0 {
      continue
    }

    let path = std::path::Path::new(component);
    
    let res = match std::fs::read_dir(component) {
      Ok(res) => res,
      Err(_) => continue
    };

    for (_, entry) in res.enumerate() {
      let entry = match entry {
        Ok(entry) => entry,
        Err(_) => continue
      };

      if !entry.file_type().unwrap().is_file() {
        continue
      }

      let file_name = entry.file_name().into_string().unwrap();
      if &file_name[0 .. name.len()] == name {
        
      }

      if cfg!(windows) {
        if &file_name[name.len() .. ] == ".exe" {
          return Some(entry.path());
        }
      } else {
        if file_name.len() == name.len() {
          return Some(entry.path());
        }
      }
    }
  }

  None
}


fn run() -> std::io::Result<()> {
  let yarn_program = find_program("yarn").unwrap();
  let webpack_program = find_program("webpack").unwrap();

  
  let mut yarn = Command::new(yarn_program)
    .arg("install")
    .current_dir("client")
    .spawn()?;

  let yarn_out = yarn.wait()?;

  let mut webpack = Command::new(webpack_program)
    .arg("-c")
    .arg("config/webpack.config.js")
    .current_dir("client")
    .spawn()?;

  let webpack_out = webpack.wait()?;
  println!("EXIT STATUS: {}", webpack_out);


  Ok(())
}

fn main() {
  run().expect("Failed to build client");
}