//! Given a `drydoc.yaml` file, generate a website.

mod fetch;
mod resource;

use std::path::PathBuf;

use clap::Clap;
use drydoc_model::{
  bundle::Bundle,
  decl::{Decl, Generate, Import},
  ns::Namespace,
};

use drydoc_pkg_manager::{Manager as PkgMgr, UrlFetcher, VersionReq};
mod actor;
mod uri;

use tokio::fs::File;

use actor::{Actor, Addr};
use std::future::Future;
use std::pin::Pin;
mod emitter;
mod generator_mgr;
mod ipc;
mod preprocessor;
mod progress;

use generator_mgr::{GeneratorMgr, GeneratorMgrMsg};

use std::error::Error;

use emitter::Emitter;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

/// Generate documentation from a drydoc.yaml file.
#[derive(Clap, Debug)]
pub struct GenOpts {
  /// The configuration file to generate from
  #[clap(short, long, default_value = "drydoc.yaml")]
  config: String,

  /// Output directory
  #[clap(short, long, default_value = "html")]
  output: String,

  #[clap(long, default_value = "https://semio-ai.github.io/drydoc-packages")]
  repository_url: String,

  #[clap(long)]
  repository_dir: Option<String>,
}

async fn gen_unit(
  config: Generate,
  mgr: Addr<GeneratorMgrMsg>,
  namespace: Arc<Namespace>,
  path: PathBuf,
) -> Result<Bundle, Box<dyn Error>> {
  let mut sub_bundles = Vec::new();

  let child_ns = namespace.child(config.id);
  if let Some(children) = config.children {
    for child in children {
      let path = path.clone();
      sub_bundles.push(gen_decl(child, mgr.clone(), child_ns.clone(), path).await?);
    }
  }

  lazy_static! {
    static ref WILDCARD: VersionReq = VersionReq::parse("*").unwrap();
  }

  let parts = config.using.split("@").collect::<Vec<&str>>();

  let (name, version_req) = if parts.len() == 1 {
    (parts[0], WILDCARD.clone())
  } else if parts.len() == 2 {
    (parts[1], VersionReq::parse(parts[2]).unwrap())
  } else {
    panic!("Invalid generator string")
  };

  let ipc = mgr.get_or_start(name, version_req).await.unwrap();

  let path = path.to_str().unwrap().to_string();
  let mut res = ipc
    .generate(0, namespace.clone(), config.with, path)
    .await?;
  for sub_bundle in sub_bundles {
    res.bundle = res.bundle.merge(sub_bundle).unwrap();
  }
  Ok(res.bundle)
}

use tokio::io::AsyncReadExt;

fn gen_decl(
  decl: Decl,
  mgr: Addr<GeneratorMgrMsg>,
  namespace: Arc<Namespace>,
  decl_path: PathBuf,
) -> Pin<Box<dyn Future<Output = Result<Bundle, Box<dyn Error>>>>> {
  Box::pin(async move {
    match decl {
      Decl::Import(Import { path }) => {
        let mut abs_path = PathBuf::new();
        abs_path.push(decl_path.parent().unwrap());
        abs_path.push(&path);
        let mut file = File::open(&abs_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let config: Decl = serde_yaml::from_str(contents.as_str()).unwrap();
        Ok(gen_decl(config, mgr.clone(), namespace, abs_path).await?)
      }
      Decl::Generate(generate) => Ok(gen_unit(generate, mgr.clone(), namespace, decl_path).await?),
    }
  })
}

use colored::*;
use log::info;

struct Logger {
  level: log::Level,
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= self.level
  }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!(
        "{}: {}",
        format!("{}", record.level()).blue(),
        record.args()
      );
    }
  }

  fn flush(&self) {}
}

async fn gen() -> Result<(), Box<dyn std::error::Error>> {
  let opts = GenOpts::parse();
  let contents = tokio::fs::read_to_string(&opts.config).await?;

  let raw_config: serde_yaml::Value = serde_yaml::from_str(contents.as_str())?;
  let decl: Decl = serde_yaml::from_value(
    preprocessor::preprocess(raw_config, Arc::new(std::env::current_dir()?)).await?,
  )?;

  log::set_logger(&Logger {
    level: log::Level::Debug,
  })
  .unwrap();

  log::set_max_level(log::LevelFilter::Debug);

  let pkg_mgr = PkgMgr::new(
    UrlFetcher::new(opts.repository_url),
    &opts.repository_dir.unwrap(),
  );
  let gen_mgr = GeneratorMgr::new(pkg_mgr).spawn();

  let bundle = gen_decl(
    decl,
    gen_mgr,
    Namespace::new("root"),
    PathBuf::from(opts.config.as_str()),
  )
  .await?;

  let emitter = emitter::html::Html::new(opts.output);
  emitter.emit(bundle).await?;

  Ok(())
}

#[tokio::main]
async fn main() {
  match gen().await {
    Err(err) => {
      eprintln!("ERROR: {:?}", err.source());
      std::process::exit(1);
    }
    _ => {
      std::process::exit(0);
    }
  }
}
