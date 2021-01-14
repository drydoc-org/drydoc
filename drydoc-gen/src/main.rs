mod config;
mod page;
mod bundle;
mod fetch;
mod resource;

use std::{collections::HashMap, path::PathBuf};

use clap::Clap;
use config::{Config, Decl, Unit};
use handlebars::Path;
use page::Id;
mod uri;
mod generator;
mod actor;
mod fs;
mod fs2;

use generator::{Generators, GeneratorsMsg, GenerateError};

use bundle::{Bundle, Manifest};
use tokio::fs::File;

use std::future::Future;
use actor::{Actor, Addr};
use std::pin::Pin;
use std::iter::FromIterator;
mod ns;

mod preprocessor;
mod progress;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

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



async fn gen_unit(unit: Unit, generators: Addr<GeneratorsMsg>, namespace: Arc<ns::Namespace>, path: PathBuf) -> Result<Bundle, GenerateError> {
  let mut sub_bundles = Vec::new();

  let child_ns = namespace.child(unit.id.0);
  if let Some(children) = unit.children {
    for child in children {
      let path = path.clone();
      sub_bundles.push(gen_decl(child, generators.clone(), child_ns.clone(), path).await?);  
    }
  }

  match generators.get(unit.rule.name.clone()).await {
    Some(generator) => {
      let mut bundle = generator.generate(unit.rule, child_ns, path).await?;
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

use tokio::io::AsyncReadExt;

fn gen_decl(decl: Decl, generators: Addr<GeneratorsMsg>, namespace: Arc<ns::Namespace>, decl_path: PathBuf) -> Pin<Box<dyn Future<Output = Result<Bundle, GenerateError>>>> {
  println!("decl: {:?} {:#?}", &decl_path, &decl);
  
  Box::pin(async move {
    match decl {
      Decl::Import(import) => {
        let mut abs_path = PathBuf::new();
        abs_path.push(decl_path.parent().unwrap());
        abs_path.push(import.uri.as_str());
        let mut file = File::open(&abs_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let config: Config = serde_yaml::from_str(contents.as_str()).unwrap();
        Ok(gen_decl(config.decl, generators.clone(), namespace, abs_path).await?)
      },
      Decl::Unit(unit) => {
        println!("unit: {:?}", &unit);
        Ok(gen_unit(unit, generators.clone(), namespace, decl_path).await?)
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
  println!("uri: {:?}", &uri);

  let contents = fetch::fetch(&uri).await?;
  let contents = String::from_utf8(contents.into()).unwrap();

  let raw_config: serde_yaml::Value = serde_yaml::from_str(contents.as_str())?;
  let decl: Config = serde_yaml::from_value(preprocessor::preprocess(raw_config, std::sync::Arc::new(std::env::current_dir()?)).await?)?;

  log::set_logger(&Logger {
    level: log::Level::Debug
  }).unwrap();

  log::set_max_level(log::LevelFilter::Debug);


  let mut generators = Generators::new();
  generators.insert_generator("copy", generator::copy::CopyGenerator::new()).await;
  generators.insert_generator("clang", generator::clang::ClangGenerator::new()).await;


  let generators = generators.spawn();
  let mut bundle = gen_decl(decl.decl, generators.clone(), ns::Namespace::new("root"), opts.config.as_str().into()).await?;

  let manifest_js = format!("window.MANIFEST = {}", serde_json::to_string_pretty(&bundle.manifest).unwrap());

  let current_exe = std::env::current_exe().unwrap();
  let home = current_exe.parent().unwrap().parent().unwrap().parent().unwrap();

  let mut js = fs2::Folder::new();
  let bundle_path = home.join(std::path::PathBuf::from_iter(&["client", "dist", "bundle.js"]));
  println!("bundle path: {:?}", &bundle_path);

  js.insert("bundle.js", fs2::RealFile::open(bundle_path)?);
  println!("inserted bundle.js");
  js.insert("manifest.js", fs2::VirtFile::new(manifest_js.as_bytes()));
  println!("inserted manifest.js");
  bundle.insert_entry("js", js);
  println!("inserted js");

  bundle.folder.merge(fs2::Folder::read(home.join("static")).await?).unwrap();

  bundle = bundle.namespace();


  println!("Writing...");
  bundle.write_out(opts.output.as_str()).await?;

  println!("Wrote out");

  Ok(())
}
