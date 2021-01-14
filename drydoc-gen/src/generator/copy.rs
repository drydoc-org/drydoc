use crate::actor::{Actor, Addr, Receiver};
// use tokio::sync::oneshot::Sender;

use super::{GeneratorMsg, GenerateError};
use crate::config::Rule;

use crate::page::{Id, Page};
use crate::bundle::{Bundle, Manifest};
use crate::fs::{File};

use crate::ns;

use std::{collections::HashMap, path::PathBuf};
use std::collections::HashSet;

use std::path::Path;
use std::sync::Arc;

mod content_type;
mod renderer;

static PARAM_PATH: &'static str = "path";
static NAME_PATH: &'static str = "name";
static HIDDEN: &'static str = "hidden";

pub struct CopyGenerator {

}

impl CopyGenerator {
  pub fn new() -> Self {
    Self {}
  }

  async fn generate(rule: Rule, namespace: &Arc<ns::Namespace>, mut path: PathBuf) -> Result<Bundle, GenerateError> {
    assert_eq!(rule.name.as_str(), "copy");

    match rule.params.get(&PARAM_PATH.to_string()) {
      Some(path_str) => {
        path.pop();
        path.push(path_str);
      },
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let hidden = match rule.params.get(&HIDDEN.to_string()) {
      Some(hidden) => match hidden.parse::<bool>() {
        Ok(hidden) => Some(hidden),
        Err(err) => return Err(GenerateError::InvalidParameter {
          name: HIDDEN.to_string(),
          message: err.to_string()
        })
      },
      None => None
    };

    let name = match rule.params.get(&NAME_PATH.to_string()) {
      Some(name) => name.to_string(),
      None => path.to_str().unwrap().to_string()
    };

    let extension = path.extension().map(|s| s.to_os_string().into_string().unwrap());

    let content_type = extension.clone()
      .and_then(|e| content_type::lookup(e))
      .unwrap_or("application/unknown")
      .to_string();

    let renderer = extension
      .and_then(|e| renderer::lookup(e))
      .unwrap_or("default")
      .to_string();

    let page_id = Id(format!("{}", namespace));
    let mut pages = HashMap::new();
    
    let url = format!("{}.{}.page", namespace, page_id);

    pages.insert(page_id.clone(), Page {
      id: page_id.clone(),
      content_type,
      name,
      renderer,
      hidden,
      resolvable: true,
      metadata: HashMap::new(),
      children: HashSet::new(),
      url: url.clone()
    });

    let mut bundle = Bundle::new(Manifest::new(page_id.clone(), pages));
    bundle.insert_entry(url, File::open(path).await?);
    Ok(bundle)
  }

  async fn run(self, mut rx: Receiver<GeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMsg::Generate { rule, namespace, path, sender } => {
          tokio::spawn(async move {
            let _ = sender.send(Self::generate(rule, &namespace, path).await);
          });
        }
      }
    }
  }
}

impl Actor for CopyGenerator {
  type Msg = GeneratorMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    tokio::spawn(self.run(rx));
    addr
  }
}