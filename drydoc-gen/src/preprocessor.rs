// use serde_yaml::{Value, Sequence, Mapping};

// use tokio::io::Result;

// use std::pin::Pin;
// use std::future::Future;


// lazy_static! {
//   pub static ref CMD_REGEX: regex::Regex = Regex::new(r"\$\((.*)\)");
// }

// async fn execute(cmd: String) -> Result<Value> {

// }

// pub fn preprocess(yaml: &Value) -> Pin<Box<dyn Future<Output = Result<Value>>>> {
//   Box::pin(async {
//     match yaml {
//       Value::Mapping(map) => {
//         for (key, value) in map.iter() {
//           let key = if let Value::String(string) = key {
//             for cap in CMD_REGEX.capture_locations(string) {
//               execute(&cap[0]).await?
//             }
//           } else {

//           }
//         }
//       },
//       Value::String(seq) => {

//       }
//     }
//   })
// }