use std::error::Error;

use clap::Clap;
use dirs::home_dir;
use drydoc_pkg_manager::{Manager, UrlFetcher};

use log::{debug, info};

#[derive(Clap, Debug)]
pub struct Opts {
  #[clap(long, default_value = "https://semio-ai.github.io/drydoc-packages")]
  url: String,

  #[clap(long)]
  repository_dir: Option<String>,

  #[clap(subcommand)]
  command: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
  Get(Get),
  Installed(Installed),
}

#[derive(Clap, Debug)]
pub struct Get {
  package: String,

  #[clap(short, long, default_value = "*")]
  version: String,
}

#[derive(Clap, Debug)]
pub struct Installed {
  #[clap(long)]
  package: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  pretty_env_logger::init();

  let Opts {
    url,
    repository_dir,
    command,
  } = Opts::parse();

  let repository_dir = if let Some(repository_dir) = repository_dir {
    repository_dir.into()
  } else {
    let mut home_dir = home_dir().expect("Unable to determine the home directory");
    home_dir.push(".drydoc");
    home_dir.push("repository");
    home_dir
  };

  debug!("Repository directory: {:?}", repository_dir);

  let mut manager = Manager::new(UrlFetcher::new(url), repository_dir);

  match command {
    Command::Get(get) => {
      manager
        .get(
          get.package.as_str(),
          &semver::VersionReq::parse(get.version.as_str())?,
        )
        .await?;
      info!("Done!");
    }
    Command::Installed(installed) => {
      let packages = manager.list_installed().await?;
      for (package_name, version) in packages {
        if let Some(name) = &installed.package {
          if &package_name != name {
            continue;
          }
        }

        println!("{}@{}", package_name, version);
      }
    }
  }

  Ok(())
}
