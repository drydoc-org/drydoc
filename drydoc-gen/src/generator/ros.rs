use crate::{actor::{Actor, Addr, Receiver}, fs2::VirtFile};
// // use tokio::sync::oneshot::Sender;

use super::{GeneratorMsg, GenerateError};
use crate::config::Rule;

use crate::page::{Id, Page};
use crate::bundle::{Bundle, Manifest};
use crate::fs2::{Entry};

use crate::ns;

use std::{collections::HashMap, path::PathBuf};
use std::collections::HashSet;

use model::{Message, Service};
use tokio::io::AsyncReadExt;

// use std::path::Path;

use std::iter::FromIterator;

use super::util::get_files;

pub mod model;

static PARAM_PATH: &'static str = "path";
static NAME_PATH: &'static str = "name";
static PACKAGE: &'static str = "package";

pub struct RosGenerator {

}

impl RosGenerator {
  pub fn new() -> Self {
    Self {}
  }

  async fn generate(rule: Rule, namespace: &ns::Namespace) -> Result<Bundle, GenerateError> {
    assert_eq!(rule.name.as_str(), "ros");

    let params = rule.params;

    let paths = match params.get(&PARAM_PATH.to_string()) {
      Some(path) => path.split(',').collect::<Vec<&str>>(),
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let package = match params.get(&PACKAGE.to_string()) {
      Some(name) => name.to_string(),
      None => return Err(GenerateError::MissingParameter(PACKAGE.to_string()))
    };

    let name = match params.get(&NAME_PATH.to_string()) {
      Some(name) => name.to_string(),
      None => package.clone()
    };

    lazy_static! {
      pub static ref VALID_EXTENSIONS: HashSet<&'static str> = HashSet::from_iter(vec![
        "msg",
        "srv"
      ]);
    }

    let mut files = Vec::new(); 
    for path in paths {
      files.extend(get_files(path, |p| VALID_EXTENSIONS.contains(p.extension().unwrap().to_str().unwrap())).await?);
    }

    let mut messages = HashMap::new();
    let mut services = HashMap::new();
    let mut actions = HashMap::new();


    let mut out_files = HashMap::new();
    for path in files {
      let path: PathBuf = path.into();
      let mut file = tokio::fs::File::open(&path).await?;
      let mut contents = String::new();
      file.read_to_string(&mut contents).await?;

      println!("{:?}: {}", &path, &contents);
      
      let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
      let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

      match path.extension().unwrap().to_str().unwrap() {
        "msg" => {
          let message = Message::parse(package.clone(), file_stem.clone(), contents).unwrap().resolve(&package);

          let id = Id(format!("{}/{}", namespace, file_stem.clone()));
          let url = format!("{}/{}.page", namespace, file_stem.clone());

          let mut metadata = HashMap::new();
          metadata.insert("section".to_string(), "message".to_string());

          out_files.insert(url.clone(), serde_json::to_vec(&message).unwrap());
          messages.insert(id.clone(), Page {
            id,
            content_type: "ros/message".to_string(),
            name: file_stem.clone(),
            renderer: "ros".to_string(),
            metadata,
            children: HashSet::new(),
            hidden: None,
            resolvable: true,
            url: url
          });
        },
        "srv" => {
          let service = Service::parse(package.clone(), file_stem.clone(), contents).unwrap().resolve(&package);

          let id = Id(format!("{}/{}", namespace, file_stem.clone()));
          let url = format!("{}/{}.page", namespace, file_stem.clone());
          
          let mut metadata = HashMap::new();
          metadata.insert("section".to_string(), "service".to_string());
          
          out_files.insert(url.clone(), serde_json::to_vec(&service).unwrap());
          services.insert(id.clone(), Page {
            id,
            content_type: "ros/service".to_string(),
            name: file_stem.clone(),
            renderer: "ros".to_string(),
            metadata,
            children: HashSet::new(),
            hidden: None,
            resolvable: true,
            url: url
          });
        },
        _ => {}
      }
    }

    let mut pages = HashMap::new();

    let mut children = HashSet::new();
    children.extend(messages.keys().map(|key| key.clone()));
    children.extend(services.keys().map(|key| key.clone()));

    let page_id = Id(namespace.to_string());
    pages.insert(Id(page_id.clone().to_string()), Page {
      id: page_id.clone(),
      content_type: "text/markdown".to_string(),
      name,
      renderer: "markdown".to_string(),
      metadata: HashMap::new(),
      children,
      hidden: None,
      resolvable: false,
      url: "".to_string()
    });

    pages.extend(messages);
    pages.extend(services);
    pages.extend(actions);

    

    let mut bundle = Bundle::new(Manifest::new(page_id.clone(), pages));

    for (key, value) in out_files {
      bundle.insert_entry(key, VirtFile::new(value.into_boxed_slice()))?;
    }

    Ok(bundle)
  }

  async fn run(self, mut rx: Receiver<GeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMsg::Generate { rule, namespace, path, sender } => {
          let _ = sender.send(Self::generate(rule, &namespace).await);
        }
      }
    }
  }
}

impl Actor for RosGenerator {
  type Msg = GeneratorMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}