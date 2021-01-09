mod config;
mod page;
mod bundle;
mod fetch;
mod resource;

use std::{collections::HashMap};

use clap::Clap;
use config::{Config, Decl, Unit};
use page::Id;
mod uri;
mod generator;
mod actor;
mod fs;
mod fs2;

use generator::{Generators, GeneratorsMsg, GenerateError};

use bundle::{Bundle, Manifest};

use std::future::Future;
use actor::{Actor, Addr};
use std::pin::Pin;
use std::iter::FromIterator;

mod preprocessor;
mod progress;

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



async fn gen_unit(unit: Unit, generators: Addr<GeneratorsMsg>, prefix: String) -> Result<Bundle, GenerateError> {
  let mut sub_bundles = Vec::new();
  if let Some(children) = unit.children {
    for child in children {
      sub_bundles.push(gen_decl(child, generators.clone(), format!("{}{}", &prefix, &unit.id)).await?);  
    }
  }

  match generators.get(unit.rule.name.clone()).await {
    Some(generator) => {
      let mut bundle = generator.generate(unit.rule, format!("{}{}", &prefix, &unit.id)).await?;
      for sub_bundle in sub_bundles {
        bundle.merge(sub_bundle).unwrap();
      }
      Ok(bundle)
    },
    None => {
      Err(GenerateError::InvalidParameter {
        name: "rule.name".to_string(),
        message: format!("{:?} doesn't match any generator", unit.rule.name)
      })
    }
  }

  
}

fn gen_decl(decl: Decl, generators: Addr<GeneratorsMsg>, prefix: String) -> Pin<Box<dyn Future<Output = Result<Bundle, GenerateError>>>> {
  Box::pin(async move {
    match decl {
      Decl::Import(import) => {
        let uri = uri::to_uri(import.uri.as_str());
        let contents = fetch::fetch(&uri).await?;
        let contents = String::from_utf8(contents.into()).unwrap();
        let config: Config = serde_yaml::from_str(contents.as_str()).unwrap();
        Ok(gen_decl(config.decl, generators.clone(), prefix).await?)
      },
      Decl::Unit(unit) => {
        println!("unit: {:?}", &unit);
        Ok(gen_unit(unit, generators.clone(), prefix).await?)
      }
    }
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

  let raw_config: serde_yaml::Value = serde_yaml::from_str(contents.as_str())?;
  let decl: Decl = serde_yaml::from_value(preprocessor::preprocess(raw_config, std::sync::Arc::new(std::env::current_dir()?)).await?)?;

  log::set_logger(&Logger {
    level: log::Level::Debug
  }).unwrap();

  log::set_max_level(log::LevelFilter::Debug);

  info!("Test");

  let mut generators = Generators::new();
  generators.insert_generator("copy", generator::copy::CopyGenerator::new()).await;
  generators.insert_generator("clang", generator::clang::ClangGenerator::new()).await;


  let generators = generators.spawn();
  let mut bundle = gen_decl(decl, generators.clone(), "".to_string()).await?;

  println!("Add static...");

  let manifest_js = format!("window.MANIFEST = {}", serde_json::to_string_pretty(&bundle.manifest).unwrap());

  let current_exe = std::env::current_exe().unwrap();
  let home = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();

  let mut js = fs::Folder::new();
  let bundle_path = home.join(std::path::PathBuf::from_iter(&["client", "dist", "bundle.js"]));
  println!("bundle path: {:?}", &bundle_path);
  js.insert("bundle.js", fs::File::open(bundle_path).await?);
  js.insert("manifest.js", fs::VirtFile::new(manifest_js.as_bytes()));

  bundle.insert_entry("js", js);
  bundle.folder.merge(&fs::Folder::read(home.join("static")).await?).unwrap();

  println!("Writing...");
  bundle.write_out(opts.output.as_str()).await?;

  println!("Wrote out");

  Ok(())
}
