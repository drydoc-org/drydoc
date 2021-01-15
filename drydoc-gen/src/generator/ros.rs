// use crate::actor::{Actor, Addr, Receiver};
// // use tokio::sync::oneshot::Sender;

// use super::{GeneratorMsg, GenerateError};
// use crate::config::Rule;

// use crate::page::{Id, Page};
// use crate::bundle::{Bundle, Manifest};
// use crate::fs::{File};

// use std::collections::HashMap;
// use std::collections::HashSet;

// use std::path::Path;

// use std::iter::FromIterator;

// use super::util::get_files;

pub mod model;

// static PARAM_PATH: &'static str = "path";
// static NAME_PATH: &'static str = "name";

// pub struct RosGenerator {

// }

// impl RosGenerator {
//   pub fn new() -> Self {
//     Self {}
//   }

//   async fn generate(rule: Rule, prefix: String) -> Result<Bundle, GenerateError> {
//     assert_eq!(rule.name.as_str(), "copy");

//     let params = rule.params;

//     let path = match params.get(&PARAM_PATH.to_string()) {
//       Some(path) => path.to_string(),
//       None => return Err(GenerateError::MissingParameter(PARAM_PATH.to_string()))
//     };

//     let name = match params.get(&NAME_PATH.to_string()) {
//       Some(name) => name.to_string(),
//       None => path.clone()
//     };

//     lazy_static! {
//       pub static ref VALID_EXTENSIONS: HashSet<&'static str> = HashSet::from_iter(vec![
//         "msg",
//         "srv"
//       ]);
//     }

//     let files = get_files(&path, |p| VALID_EXTENSIONS.contains(p.extension().unwrap().to_str().unwrap())).await?;

//     for file in files {
//       let mut file = tokio::fs::File::open(file).await?;
//       // file.read_to_string(&mut )
//     }

//     let page_id = Id(format!("{}", prefix));
//     let mut pages = HashMap::new();
    
//     pages.insert(page_id.clone(), Page {
//       id: page_id.clone(),
//       content_type: "text/markdown".to_string(),
//       name,
//       renderer: "markdown".to_string(),
//       metadata: HashMap::new(),
//       children: HashSet::new()
//     });



//     let mut bundle = Bundle::new(Manifest::new(page_id.clone(), pages));
//     bundle.insert_entry(format!("{}.page", page_id), File::open(path).await?);
//     Ok(bundle)
//   }

//   async fn run(self, mut rx: Receiver<GeneratorMsg>) {
//     while let Some(msg) = rx.recv().await {
//       match msg {
//         GeneratorMsg::Generate { rule, prefix, sender } => {
//           let _ = sender.send(Self::generate(rule, prefix).await);
//         }
//       }
//     }
//   }
// }

// impl Actor for RosGenerator {
//   type Msg = GeneratorMsg;

//   fn spawn(self) -> Addr<Self::Msg> {
//     let (addr, rx) = Addr::new();
//     tokio::spawn(self.run(rx));
//     addr
//   }
// }