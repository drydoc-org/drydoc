use clang::*;

use crate::actor::{Actor, Addr, Receiver};

use super::{GeneratorMsg, GenerateError};
use crate::config::Unit;
use crate::page::{Page, Id};
use crate::fs::{VirtFile};
use crate::bundle::{Bundle, Manifest};

use std::path::{Path, PathBuf};

use std::pin::Pin;
use std::future::Future;

use std::collections::{HashMap, HashSet};

mod model;

use model::{EntityLike};

use serde::{Serialize, Deserialize};

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



  fn get_files<P: 'static + AsRef<Path>>(path: P) -> Pin<Box<dyn Future<Output = tokio::io::Result<Vec<PathBuf>>>>> {
    Box::pin(async move {
      let path = path.as_ref();

      if path.is_dir() {
        let mut dir = tokio::fs::read_dir(path).await?;

        let mut ret = Vec::new();
        while let Ok(Some(entry)) = dir.next_entry().await {
          let subpaths = Self::get_files(entry.path()).await?;
          ret.extend_from_slice(subpaths.as_slice());
        }

        Ok(ret)
      } else {
        Ok(vec![ path.to_path_buf() ])
      }
    })
  }

  fn to_pages(prefix: String, symbols: &HashMap<String, model::Entity>) -> HashMap<Id, Page> {
    let mut ret = HashMap::with_capacity(symbols.len());    
    for (_, entity) in symbols.iter() {
      let page = entity.to_page(prefix.clone(), symbols);
      ret.insert(page.id.clone(), page);
    }
    ret
  }

  async fn generate(unit: Unit, prefix: String) -> Result<Bundle, GenerateError> {
    let clang = clang::Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    let params = unit.rule.params;

    let path = match params.get(&PARAM_PATH.to_string()) {
      Some(path) => path.clone(),
      None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
    };

    let name = match params.get(&NAME_PATH.to_string()) {
      Some(name) => name.to_string(),
      None => path.to_string()
    };

    let arguments = match params.get(&"arguments".to_string()) {
      Some(arguments) => arguments.split(' ').collect::<Vec<&str>>(),
      None => Vec::new()
    };

    let paths = Self::get_files(path).await?;

    let mut symbols = HashMap::new();
    let mut roots = HashSet::new();
    for path in paths.into_iter() {
      let tu = index.parser(path)
        .incomplete(true)
        .skip_function_bodies(true)
        .arguments(arguments.as_slice())

       // .single_file_parse(true)
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
        GeneratorMsg::Generate { unit, prefix, sender } => {
          let _ = sender.send(Self::generate(unit, prefix).await);
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
