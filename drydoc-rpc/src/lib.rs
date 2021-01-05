// use serde::{Serialize, Deserialize};

// use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
// use std::collections::{HashMap, VecDeque};
// use tokio::sync::oneshot::{Receiver, Sender, channel};

// #[derive(Serialize, Deserialize)]
// pub struct Request<T> {
//   id: u64,
//   data: T
// }

// #[derive(Serialize, Deserialize)]
// pub struct Response<T> {
//   id: u64,
//   data: T
// }

// #[derive(Serialize, Deserialize)]
// pub enum Msg<Req, Res> {
//   Request(Request<Req>),
//   Response(Response<Res>)
// }


// pub struct Client<Req, Res, R: AsyncRead, W: AsyncWrite> {
//   req_marker: std::marker::PhantomData<Req>,
  
//   input: R,
//   output: W,

//   outstanding: HashMap<u64, Sender<Res>>,
//   id_iter: u64,

//   incoming: VecDeque<u8>
// }

// impl<'de, Req, Res, R, W> Client<Req, Res, R, W>
// where
//   Req: Serialize,
//   Res: Deserialize<'de>,
//   R: Unpin + AsyncRead,
//   W: Unpin + AsyncWrite
// {
//   pub fn new((input, output): (R, W)) -> Self {
//     Self {
//       req_marker: std::marker::PhantomData {},
//       input,
//       output,
//       outstanding: HashMap::new(),
//       id_iter: 0
//     }
//   }

//   pub async fn request(&mut self, req: Req) -> tokio::io::Result<Receiver<Result<Res, ()>>> {
//     let id = self.id_iter;
//     self.id_iter += 1;

//     let req = Request {
//       id,
//       data: req
//     };

//     let ser = bincode::serialize(&req).unwrap();

//     let (tx, rx) = channel();
//     self.output.write_all(ser.as_slice()).await?;

//     self.outstanding.insert(id, tx);

//     Ok(rx)
//   }

//   pub async fn update(&mut self) -> tokio::io::Result<()> {
//     let mut buf = [0u8; 512];

//     self.input.read(&mut buf);
//   }
// }