use clang::*;

use crate::actor::{Actor, Addr, Receiver};

use super::{GeneratorMsg, GenerateError};
use crate::config::Rule;
use crate::page::{Page, Id};
use crate::fs::{VirtFile};
use crate::bundle::{Bundle, Manifest};

use std::path::{Path, PathBuf};

use std::pin::Pin;
use std::future::Future;

use std::collections::{HashMap, HashSet};

mod model;

use model::{EntityLike};

use super::util::get_files;

use serde::{Serialize, Deserialize};

use std::iter::FromIterator;

static PARAM_PATH: &'static str = "path";
static NAME_PATH: &'static str = "name";


pub struct ClangGenerator {

}

#[derive(Serialize, Debug)]
pub struct PageData<'a> {
  name: String,
  symbols: HashMap<&'a String, &'a model::Entity>
}

impl ClangGenerator {
  pub fn new() -> Self {
    Self {}
  }



  

  fn to_pages(prefix: String, symbols: &HashMap<String, model::Entity>) -> HashMap<Id, Page> {
    let mut ret = HashMap::with_capacity(symbols.len());    
    for (_, entity) in symbols.iter() {
      let page = entity.to_page(prefix.clone(), symbols);
      ret.insert(page.id.clone(), page);
    }
    ret
  }

  async fn generate(rule: Rule, prefix: String) -> Result<Bundle, GenerateError> {
    let clang = clang::Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    let params = rule.params;

    let path = match params.get(&PARAM_PATH.to_string()) {
      Some(path) => path.clone(),
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let name = params
      .get(&NAME_PATH.to_string())
      .unwrap_or(&path);

    let arguments = params
      .get(&"arguments".to_string())
      .map(|args| args.split(' ').collect::<Vec<&str>>())
      .unwrap_or(Vec::new());

    lazy_static! {
      pub static ref VALID_EXTENSIONS: HashSet<&'static str> = HashSet::from_iter(vec![
        "h",
        "hh",
        "h++",
        "hpp",
        "hxx",
      ]);
    }

    let paths = get_files(path.as_str(), |p| VALID_EXTENSIONS.contains(p.extension().unwrap().to_str().unwrap())).await?;

    let mut symbols = HashMap::new();
    let mut roots = HashSet::new();
    for path in paths.into_iter() {
      let tu = index.parser(path)
        .incomplete(true)
        .skip_function_bodies(true)
        .arguments(arguments.as_slice())
        .parse();

      let tu = match tu {
        Ok(tu) => tu,
        Err(err) => return Err(GenerateError::Internal(Box::new(err)))
      };

      let mut mangler = model::Mangler::new();
      roots.extend(model::Entity::visit(tu.get_entity(), &mut mangler, &mut symbols).into_iter());
    }

    let mut root_page = Page::builder()
      .id(format!("{}", prefix))
      .name(name)
      .content_type("clang/home")
      .renderer("clang")
      .build()
      .unwrap();

    for root in roots.iter() {
      root_page.children.insert(Id(format!("{}{}", prefix.as_str(), root)));
    }

    let mut pages = Self::to_pages(prefix.clone(), &symbols);

    let root_id = root_page.id.clone();
    pages.insert(root_page.id.clone(), root_page);

    let mut bundle = Bundle::new(Manifest::new(root_id, pages));
    for (name, entity) in symbols.iter() {
      let mut names = entity.children(&symbols).unwrap_or(HashSet::new());
      if let Some(linked) = entity.linked(&symbols) {
        names.extend(linked);
      }
      names.insert(name.clone());
      
      let data = PageData {
        name: name.clone(),
        symbols: model::subset(&symbols, names)
      };
      let entity_json = serde_json::to_vec(&data).unwrap();
      bundle.insert_entry(format!("{}{}.page", prefix.as_str(), name), VirtFile::new(entity_json));
    }

    Ok(bundle)
  }
  
  async fn run(self, mut rx: Receiver<GeneratorMsg>) {
    while let Some(msg) = rx.recv().await {
      match msg {
        GeneratorMsg::Generate { rule, prefix, sender } => {
          let _ = sender.send(Self::generate(rule, prefix).await);
        }
      }
    }
  }
}


impl Actor for ClangGenerator {
  type Msg = GeneratorMsg;

  fn spawn(self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    std::thread::spawn(move || {
      let mut rt = tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
        
      let _guard = rt.enter();
      rt.block_on(async move {
        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
          tokio::task::spawn_local(self.run(rx)).await.unwrap();
        }).await;
      });
    });
    addr
  }
}
