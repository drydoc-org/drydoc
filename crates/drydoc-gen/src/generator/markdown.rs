use crate::actor::{Actor, Addr, Receiver};
// use tokio::sync::oneshot::Sender;

use super::{GeneratorMsg, GenerateError};
use crate::config::Rule;

use crate::page::{Id, Page};
use crate::bundle::{Bundle, Manifest};
use crate::fs::{File};

use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;

static PARAM_PATH: &'static str = "path";
static NAME_PATH: &'static str = "name";

pub struct MarkdownGenerator {

}

impl MarkdownGenerator {
  pub fn new() -> Self {
    Self {}
  }

  async fn generate(rule: Rule, prefix: String) -> Result<Bundle, GenerateError> {
    assert_eq!(rule.name.as_str(), "markdown");

    let path = match rule.params.get(&PARAM_PATH.to_string()) {
      Some(path) => Path::new(path),
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let name = match rule.params.get(&NAME_PATH.to_string()) {
      Some(name) => name.to_string(),
      None => path.to_str().unwrap().to_string()
    };

    

    let page_id = Id(format!("{}", prefix));
    let mut pages = HashMap::new();
    
    pages.insert(page_id.clone(), Page {
      id: page_id.clone(),
      content_type: "text/markdown".to_string(),
      name,
      renderer: "markdown".to_string(),
      metadata: HashMap::new(),
      children: HashSet::new()
    });

    let mut file = tokio::fs::File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    lazy_static! {
      pub static ref LINK_REGEX: HashMap<&'static str, &'static str> = map! {
        "md" => "text/markdown",
        "markdown" => "text/markdown",
        "cpp" => "text/cpp",
        "cxx" => "text/cpp",
        "c" => "text/c",
        "hpp" => "text/hpp",
        "hxx" => "text/hpp",
        "h" => "text/h"
      };
    }
    

    let mut bundle = Bundle::new(Manifest::new(page_id.clone(), pages));
    bundle.insert_entry(format!("{}.page", page_id), );
    Ok(bundle)
  }

  async fn run(self, mut rx: Receiver<GeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMsg::Generate { rule, prefix, sender } => {
          tokio::spawn(async move {
            let _ = sender.send(Self::generate(rule, prefix).await);
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