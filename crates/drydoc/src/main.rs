use std::env::{args, current_exe};
use std::fs;
use std::io::Result;
use std::process::{Command, Stdio};

fn run() -> Result<()> {
  let current_exe = current_exe()?;
  let exe_dir = current_exe.parent().unwrap();
  let args: Vec<String> = args().skip(1).collect();

  if let Some((head, tail)) = args.split_first() {
    let mut path = exe_dir.to_path_buf();
    path.push(format!(
      "drydoc-{}{}",
      head,
      if cfg!(windows) { ".exe" } else { "" }
    ));

    let cmd = Command::new(path)
      .args(tail)
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()?;

    cmd.wait_with_output()?;
  } else {
    for entry in fs::read_dir(exe_dir)? {
      let entry = entry?;
      let path = entry.path();
    }
  }

  Ok(())
}

fn main() {
  run().unwrap();
}
