use drydoc_pkg_manager::Version;

/// Link a local package into the package repository for testing
#[derive(Clap, Debug)]
pub struct Link {

  /// The name of the package (from drydoc's perspective)
  #[clap(short, long)]
  name: String,

  /// The version of the package (from drydoc's perspective)
  #[clap(short, long)]
  version: Version,

  #[clap(short, long)]
  dir: String
}
#[derive(Clap, Debug)]
pub enum Command {
  Link(Link)
}

#[derive(Clap, Debug)]
pub struct DevOpts {
  #[clap(short, long, default_value = "html")]
  dir: String,

  #[clap(short, long, default_value = "127.0.0.1")]
  address: String,

  #[clap(short, long, default_value = "8888")]
  
}

fn main() -> {

}