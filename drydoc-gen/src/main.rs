mod config;
mod page;
mod bundle;
mod fetch;
mod resource;

use std::{collections::HashMap};

use clap::Clap;
use config::{Config, Decl, Unit};
use page::Id;
use url::Url;
mod uri;
mod generator;
mod actor;
mod fs;

use generator::{Generators, GeneratorsMsg, GenerateError};

use bundle::{Bundle, Manifest};

use std::future::Future;
use actor::{Actor, Addr};
use std::pin::Pin;
use std::iter::FromIterator;

mod preprocessor;

#[macro_use]
extern crate lazy_static;


/// Generate documentation
#[derive(Clap, Debug)]
pub struct GenOpts {
  /// The configuration file to generate from
  #[clap(short, long, default_value = "drydoc.yaml")]
  config: String,

  /// Output directory
  #[clap(short, long, default_value = "html")]
  output: String
}



async fn gen_unit(unit: Unit, generators: Addr<GeneratorsMsg>) -> Result<Bundle, GenerateError> {
  match generators.get(unit.rule.name.clone()).await {
    Some(generator) => {
      generator.generate(unit, "".to_string()).await
    },
    None => {
      Err(GenerateError::InvalidParameter {
        name: "rule.name".to_string(),
        message: format!("{:?} doesn't match any generator", unit.rule.name)
      })
    }
  }
}

fn gen_config(config: Config, generators: Addr<GeneratorsMsg>) -> Pin<Box<dyn Future<Output = tokio::io::Result<Bundle>>>> {
  Box::pin(async move {

    

    let mut ret = Bundle {
      manifest: Manifest {
        root: Id("".to_string()),
        pages: HashMap::new()
      },
      folder: fs::Folder::new()
    };

    for decl in config.decls.into_iter() {
      match decl {
        Decl::Import(import) => {
          let uri = uri::to_uri(import.uri.as_str());
          let contents = fetch::fetch(&uri).await?;
          let contents = String::from_utf8(contents.into()).unwrap();
          let unit: Config = serde_yaml::from_str(contents.as_str()).unwrap();
          let bundle = gen_config(unit, generators.clone()).await?;
          ret.merge(bundle).unwrap();
        },
        Decl::Unit(unit) => {
          println!("unit: {:?}", &unit);

          let bundle = gen_unit(unit, generators.clone()).await.unwrap();
          ret.merge(bundle).unwrap();
        }
      }
    }

    Ok(ret)
  })
}

use colored::*;
use log::info;

struct Logger {
  level: log::Level
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= self.level
  }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!("{}: {}", format!("{}", record.level()).blue(), record.args());
    }
  }

  fn flush(&self) {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opts = GenOpts::parse();
  let uri = uri::to_uri(opts.config.as_str());
  let contents = fetch::fetch(&uri).await?;
  let contents = String::from_utf8(contents.into()).unwrap();
  let config: Config = serde_yaml::from_str(contents.as_str()).unwrap();

  log::set_logger(&Logger {
    level: log::Level::Debug
  });

  log::set_max_level(log::LevelFilter::Debug);

  info!("Test");

  let mut generators = Generators::new();
  generators.insert_generator("copy", generator::copy::CopyGenerator::new()).await;
  generators.insert_generator("clang", generator::clang::ClangGenerator::new()).await;


  let generators = generators.spawn();
  let mut bundle = gen_config(config, generators.clone()).await?;

  println!("Add static...");

  let manifest_js = format!("window.MANIFEST = {}", serde_json::to_string_pretty(&bundle.manifest).unwrap());

  let current_exe = std::env::current_exe().unwrap();
  let home = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();

  let mut js = fs::Folder::new();
  let bundle_path = home.join(std::path::PathBuf::from_iter(&["client", "dist", "bundle.js"]));
  println!("bundle path: {:?}", &bundle_path);
  js.insert("bundle.js", fs::File::open(bundle_path).await.unwrap());
  js.insert("manifest.js", fs::VirtFile::new(manifest_js.as_bytes()));

  bundle.insert_entry("js", js);
  bundle.folder.merge(&fs::Folder::read(home.join("static")).await.unwrap()).unwrap();

  println!("Writing...");
  bundle.write_out(opts.output.as_str()).await.unwrap();

  println!("Wrote out");

  Ok(())
}
