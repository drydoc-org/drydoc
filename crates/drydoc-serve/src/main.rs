use clap::Clap;


#[derive(Clap, Debug)]
pub struct ServeOpts {
  #[clap(short, long, default_value = "html")]
  dir: String,

  #[clap(short, long, default_value = "127.0.0.1")]
  address: String,

  #[clap(short, long, default_value = "8888")]
  port: u16,
}

#[tokio::main]
async fn main() {
  let opts = ServeOpts::parse();
  let hello = warp::fs::dir(opts.dir);

  warp::serve(hello)
    .run((opts.address.parse::<std::net::IpAddr>().unwrap(), opts.port))
    .await;
}