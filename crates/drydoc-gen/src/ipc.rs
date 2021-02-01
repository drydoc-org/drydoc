/// IPC communication with external documentation generators
use std::{
  collections::HashMap,
  iter::FromIterator,
  net::{Ipv4Addr, SocketAddrV4},
  path::PathBuf,
};

use crate::actor::{
  map::{Map, Msg as MapMsg},
  store::{Msg as StoreMsg, Store},
  Actor, Addr, Receiver, WeakAddr,
};
use client::RequestData;
use drydoc_model::{client, ns::Namespace, server, Encoding, Message};
use log::error;
use tokio::{
  io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
  net::TcpSocket,
};

use tokio::sync::oneshot::{channel, Sender};

use drydoc_ipc::MessageProcessor;

use derive_more::{Display, Error};

use std::sync::Arc;

type ResponseSender<T> = Sender<Result<T, Error>>;

static IPC_VERSION: u32 = 1;

#[derive(Display, Debug, Error)]
pub enum Error {}

pub struct Init {
  pub res: Sender<Result<(), Error>>,
}

/// A request to open a context.
pub struct OpenContext {
  pub id: u32,
  pub res: ResponseSender<client::OpenContextResponse>,
}

/// A request to close a given context, returning a final context bundle.
pub struct CloseContext {
  pub id: u32,
  pub res: ResponseSender<client::CloseContextResponse>,
}

/// A request to the external documentation generator to generate a bundle.
pub struct Generate {
  context_id: u32,
  namespace: String,
  params: HashMap<String, String>,
  path: String,
  pub res: ResponseSender<client::GenerateResponse>,
}

pub enum IpcMsg {
  Generate(Generate),
  Init(Init),
  OpenContext(OpenContext),
  CloseContext(CloseContext),
}

impl From<Generate> for IpcMsg {
  fn from(value: Generate) -> Self {
    Self::Generate(value)
  }
}

impl From<Init> for IpcMsg {
  fn from(value: Init) -> Self {
    Self::Init(value)
  }
}

impl From<OpenContext> for IpcMsg {
  fn from(value: OpenContext) -> Self {
    Self::OpenContext(value)
  }
}

impl From<CloseContext> for IpcMsg {
  fn from(value: CloseContext) -> Self {
    Self::CloseContext(value)
  }
}

enum IpcInternalMsg {
  Ipc(IpcMsg),
  Init(client::InitializeResponse),
  Send(Message),
}

impl From<IpcMsg> for IpcInternalMsg {
  fn from(value: IpcMsg) -> Self {
    Self::Ipc(value)
  }
}

trait Responder {
  fn resolve(self: Box<Self>, data: client::ResponseData) -> Result<(), ()>;
}

struct ResponderMapper<T, F>
where
  F: FnOnce(client::ResponseData) -> Option<T>,
{
  sender: Sender<T>,
  f: F,
}

impl<T, F> ResponderMapper<T, F>
where
  F: FnOnce(client::ResponseData) -> Option<T>,
{
  pub fn new(sender: Sender<T>, f: F) -> Self {
    Self { sender, f }
  }
}

impl<T, F> Responder for ResponderMapper<T, F>
where
  F: FnOnce(client::ResponseData) -> Option<T>,
{
  fn resolve(self: Box<Self>, data: client::ResponseData) -> Result<(), ()> {
    // If this fails to unwrap it means a programming
    // mistake was made. Hard error.
    let res = (self.f)(data).unwrap();
    self.sender.send(res).map_err(|_| ())
  }
}

pub struct Ipc<R, W>
where
  R: 'static + AsyncRead + Send + Unpin,
  W: 'static + AsyncWrite + Send + Unpin,
{
  write: W,
  read: Option<R>,
  outstanding_requests: Addr<StoreMsg<u64, Box<dyn Responder + Send + Sync>>>,
  request_id_iter: u64,
  encoding: Encoding,
  drops: Vec<Box<dyn 'static + Drop + Send>>,
}

struct IpcReader<R>
where
  R: 'static + AsyncRead + Send + Unpin,
{
  addr: WeakAddr<IpcInternalMsg>,
  read: R,
  outstanding_requests: Addr<StoreMsg<u64, Box<dyn Responder + Send + Sync>>>,
}

impl<R, W> Ipc<R, W>
where
  R: 'static + AsyncRead + Send + Unpin,
  W: 'static + AsyncWrite + Send + Unpin,
{
  pub fn new(read: R, write: W) -> Self {
    Self {
      read: Some(read),
      write,
      outstanding_requests: Store::new().spawn(),
      request_id_iter: 0,
      encoding: Encoding::Json,
      drops: Vec::new(),
    }
  }

  pub fn add_drop(&mut self, drop: impl Drop + Send + 'static) {
    self.drops.push(Box::new(drop));
  }

  async fn on_event(this: &mut IpcReader<R>, event: client::Event) {}

  async fn on_request(this: &mut IpcReader<R>, request: client::Request) {
    use client::{OpenRequest, ReleaseRequest, Request};

    let Request { id, data } = request;

    match data {
      RequestData::Open(OpenRequest { path }) => {}
      RequestData::Release(ReleaseRequest { handle }) => {}
    }
  }

  async fn on_response(this: &mut IpcReader<R>, response: client::Response) {
    let client::Response { id, data } = response;
    if let Some(responder) = this.outstanding_requests.remove(id).await.unwrap() {
      if let Err(_) = responder.resolve(data) {
        error!("Failed to resolve response id {}", id)
      }
    }
  }

  async fn process_message(this: &mut IpcReader<R>, message: Message) {
    let encoding = message.encoding().expect("Invalid encoding");
    let message: client::MessageData = match encoding {
      Encoding::Json => serde_json::from_slice(message.data()).expect("Invalid JSON"),
      Encoding::Pickle => serde_pickle::from_slice(message.data()).expect("Invalid Pickle"),
      Encoding::Bincode => bincode::deserialize(message.data()).expect("Invalid Bincode"),
    };

    use client::MessageData;

    match message {
      MessageData::Event(evt) => Self::on_event(this, evt).await,
      MessageData::Request(req) => Self::on_request(this, req).await,
      MessageData::Response(res) => Self::on_response(this, res).await,
    }
  }

  async fn read(mut this: IpcReader<R>) {
    let mut processor = MessageProcessor::new();
    let mut buf = [0u8; 512];
    while let Ok(size) = this.read.read(&mut buf).await {
      if size == 0 {
        // FIXME: There's probably some way to not poll
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        continue;
      }

      processor.submit(&buf[..size]);
      while let Some(message) = processor.next() {
        Self::process_message(&mut this, message).await
      }
    }
  }

  async fn open_context(&mut self, open: OpenContext) {
    use server::{OpenContextRequest, Request};

    let OpenContext { id, res } = open;

    let req = OpenContextRequest { id };

    let responder = ResponderMapper::new(res, |data| {
      if let client::ResponseData::OpenContext(ctx) = data {
        Some(Ok(ctx))
      } else {
        None
      }
    });

    self.request_id_iter += 1;
    self
      .outstanding_requests
      .insert(self.request_id_iter, Box::new(responder))
      .await
      .unwrap();

    self
      .write_message(Request {
        id: self.request_id_iter,
        data: req.into(),
      })
      .await;
  }

  async fn close_context(&mut self, close: CloseContext) {
    use server::{CloseContextRequest, Request};

    let CloseContext { id, res } = close;

    let req = CloseContextRequest { id };

    let responder = ResponderMapper::new(res, |data| {
      if let client::ResponseData::CloseContext(ctx) = data {
        Some(Ok(ctx))
      } else {
        None
      }
    });

    self.request_id_iter += 1;
    self
      .outstanding_requests
      .insert(self.request_id_iter, Box::new(responder))
      .await
      .unwrap();

    self
      .write_message(Request {
        id: self.request_id_iter,
        data: req.into(),
      })
      .await;
  }

  async fn generate(&mut self, generate: Generate) {
    use server::{GenerateRequest, Request};

    let Generate {
      context_id,
      namespace,
      params,
      path,
      res,
    } = generate;

    let req = GenerateRequest {
      context_id,
      params,
      path,
    };

    let responder = ResponderMapper::new(res, |data| {
      if let client::ResponseData::Generate(ctx) = data {
        Some(Ok(ctx))
      } else {
        None
      }
    });

    self.request_id_iter += 1;
    self
      .outstanding_requests
      .insert(self.request_id_iter, Box::new(responder))
      .await
      .unwrap();

    self
      .write_message(Request {
        id: self.request_id_iter,
        data: req.into(),
      })
      .await;
  }

  async fn write_message<T: Into<server::MessageData>>(&mut self, msg: T) {
    let msg = Message::encode(self.encoding, &msg.into()).unwrap();

    self.write.write_all(msg.raw()).await.unwrap();
  }

  async fn run(mut self, mut rx: Receiver<IpcInternalMsg>) {
    let mut inited = false;

    while let Some(msg) = rx.recv().await {
      match msg {
        IpcInternalMsg::Send(msg) => {
          self.write.write_all(msg.raw()).await.unwrap();
        }
        IpcInternalMsg::Init(res) => {
          inited = true;
          self.encoding = res.encoding;
        }
        IpcInternalMsg::Ipc(ipc) => {
          if !inited {
            panic!("Not inited");
          }

          match ipc {
            IpcMsg::OpenContext(open) => self.open_context(open).await,
            IpcMsg::CloseContext(close) => self.close_context(close).await,
            IpcMsg::Generate(close) => self.generate(close).await,
            _ => {}
          }
        }
      }
    }
  }
}

impl<R, W> Actor for Ipc<R, W>
where
  R: AsyncRead + Send + Unpin,
  W: AsyncWrite + Send + Unpin,
{
  type Msg = IpcMsg;

  fn spawn(mut self) -> Addr<Self::Msg> {
    let (addr, rx) = Addr::new();
    let read = self.read.take().unwrap();
    let outstanding_requests = self.outstanding_requests.clone();
    let map = tokio::spawn(self.run(rx));
    tokio::spawn(Self::read(IpcReader {
      read,
      addr: addr.downgrade(),
      outstanding_requests,
    }));
    addr.upcast()
  }
}

impl Addr<IpcMsg> {
  pub async fn open_context(&self, id: u32) -> Result<client::OpenContextResponse, Error> {
    let (tx, rx) = channel();
    self.send(OpenContext { id, res: tx }).unwrap();
    rx.await.unwrap()
  }

  pub async fn close_context(&self, id: u32) -> Result<client::CloseContextResponse, Error> {
    let (tx, rx) = channel();
    self.send(CloseContext { id, res: tx }).unwrap();
    rx.await.unwrap()
  }

  pub async fn init(&self) -> Result<(), Error> {
    let (tx, rx) = channel();
    self.send(Init { res: tx }).unwrap();
    rx.await.unwrap()
  }

  pub async fn generate(
    &self,
    context_id: u32,
    namespace: Arc<Namespace>,
    params: HashMap<String, String>,
    path: String,
  ) -> Result<client::GenerateResponse, Error> {
    let (tx, rx) = channel();
    self
      .send(Generate {
        context_id,
        params,
        namespace: namespace.to_string(),
        path,
        res: tx,
      })
      .unwrap();
    rx.await.unwrap()
  }
}

use tokio::{
  net::TcpStream,
  process::{ChildStdin, ChildStdout, Command},
};

use std::{path::Path, process::Stdio, time::Duration};

use drydoc_pkg_manager::{GeneratorArtifact, IpcChannel};

pub async fn pipe(stdout: ChildStdout, stdin: ChildStdin) -> Addr<IpcMsg> {
  Ipc::new(stdout, stdin).spawn()
}

pub async fn tcp(stream: TcpStream) -> Addr<IpcMsg> {
  let (rx, tx) = stream.into_split();
  Ipc::new(rx, tx).spawn()
}

pub async fn start_generator<P: AsRef<Path>>(
  path: P,
  artifact: &GeneratorArtifact,
) -> std::io::Result<Addr<IpcMsg>> {
  let mut program_path = path.as_ref().to_path_buf();
  program_path.push(&artifact.entrypoint);

  let mut cmd = Command::new(program_path);
  cmd.kill_on_drop(true);

  match artifact.ipc_channel {
    IpcChannel::Stdio => {
      cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    }
    IpcChannel::Tcp { .. } => {
      cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit());
    }
  }

  let child = cmd.spawn()?;

  match artifact.ipc_channel {
    IpcChannel::Stdio => Ok(pipe(child.stdout.unwrap(), child.stdin.unwrap()).await),
    IpcChannel::Tcp { port } => {
      lazy_static! {
        static ref LOCAL_HOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
      }

      // Wait 500ms for the socket to become available.
      // FIXME: We should clean this up.
      tokio::time::sleep(Duration::from_millis(500)).await;
      let socket = TcpSocket::new_v4()?;
      let addr = SocketAddrV4::new(*LOCAL_HOST, port);
      Ok(tcp(socket.connect(addr.into()).await?).await)
    }
  }
}
