// use std::error::Error;

// use clap::Clap;

// use dirs::home_dir;

// use std::fs::{read_dir};

// use warp::{
//   http::{Response, StatusCode},
//   Filter,
// };

// #[derive(Clap, Debug)]
// pub struct Opts {
//   #[clap(short, long, default_value = "127.0.0.1")]
//   address: String,

//   #[clap(short, long, default_value = "8374")]
//   port: u16,

//   #[clap(long)]
//   repository_dir: Option<String>
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//   pretty_env_logger::init();

//   let Opts { address, port, repository_dir } = Opts::parse();

//   let repository_dir = if let Some(repository_dir) = repository_dir {
//     repository_dir.into()
//   } else {
//     let mut home_dir = home_dir().expect("Unable to determine the home directory");
//     home_dir.push(".drydoc");
//     home_dir.push("repository");
//     home_dir
//   };

  

//   let address = address.parse::<std::net::IpAddr>().expect("Invalid address");

//   let root = warp::get()
//     .and(warp::path(""))
//     .map(|| {
//       let dir = match read_dir(repository_dir) {
//         Ok(dir) => {

//         },
//         Err()
//       }
//       Response::builder()
//         .header(key, value)
//         .body("Test")
//     });


//   warp::serve(root)
//     .run((address, port))
//     .await;

//   Ok(())
// }