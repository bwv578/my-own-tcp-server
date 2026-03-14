use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::{io, thread};
use std::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use mio::{Events, Token};
use mio::net::{TcpListener, TcpStream};


pub trait AsyncProtocol: Send + Sync + 'static {
    fn handle_async_connection(&self, stream: TcpStream, peer: SocketAddr) -> impl Future<Output = ()> + Send;
}


pub struct AsyncTcpStream {
    stream: TcpStream,
    token: Token,
    read_buf: Vec<u8>,
    waker_vtable: Arc<Mutex<HashMap<Token, Waker>>>,
}
impl AsyncTcpStream {
    pub fn new(stream: TcpStream, token: Token, waker_vtable:Arc<Mutex<HashMap<Token, Waker>>>) -> Self {
        Self { stream, token, read_buf: Vec::new(), waker_vtable }
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    fn poll_load_buf(&mut self, cx:&mut Context) -> Poll<io::Result<usize>> {
        let mut chunk = [0u8; 4096];
        match self.stream.read(&mut chunk) {
            Ok(n) => {
                self.read_buf.extend_from_slice(&chunk[..n]);
                Poll::Ready(Ok(n))
            },
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                let mut waker_store = self.waker_vtable.lock().unwrap();
                waker_store.insert(self.token.clone(), cx.waker().clone());
                Poll::Pending
            },
            Err(e) => Poll::Ready(Err(e)),
        }
    }
    pub fn load_buf(&mut self) -> impl Future<Output = io::Result<usize>> + '_ {
        std::future::poll_fn(move |cx| self.poll_load_buf(cx))
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_buf.len() >= buf.len() {
            buf.copy_from_slice(&self.read_buf[..buf.len()]);
            self.read_buf.drain(..buf.len());
            return Ok(buf.len());
        }

        while self.read_buf.len() < buf.len() {
            if self.load_buf().await? == 0 {break;}
        }

        let available = std::cmp::min(self.read_buf.len(), buf.len());
        buf.copy_from_slice(&self.read_buf[..available]);
        self.read_buf.drain(..available);
        Ok(available)
    }

    /*pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
    }
    pub async fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
    }*/
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }
}


pub struct EventManager {
    event_queue: Events,
    poll: mio::Poll,
}
impl EventManager {
    pub fn new() -> Self {
        Self {
            event_queue: Events::with_capacity(1024),
            poll: mio::Poll::new().unwrap(),
        }
    }
}


struct TaskQueue {
    queue: ArrayQueue<AsyncTask>,
    empty: Mutex<bool>,
    notifier: Condvar
}
impl TaskQueue {
    fn new() -> Self {
        Self {
            queue: ArrayQueue::new(512),
            empty: Mutex::new(true),
            notifier: Condvar::new()
        }
    }

    fn push(&self, task: AsyncTask) {
        let mut empty = self.empty.lock().unwrap();
        match self.queue.push(task) {
            Ok(_) => {}
            Err(_) => {}
        }
        *empty = false;
    }

    fn pop(&self) -> Option<AsyncTask> {
        let mut empty =self.empty.lock().unwrap();
        while *empty {
            empty = self.notifier.wait(empty).unwrap();
        }
        self.queue.pop()
    }
}


struct Worker {
    task_queue: Arc<TaskQueue>,
}
impl Worker {
    fn spawn(mut self) {
        self.task_queue = Arc::new(TaskQueue::new());
        let queue = Arc::clone(&self.task_queue);
        thread::spawn(async move || {
            loop {
                let task = queue.pop().unwrap();
                task.await;
            }
        });
    }
}


struct ThreadPool {
    workers: Vec<Worker>,
    round: usize,
}
impl ThreadPool {
    fn new() -> Self {
        Self {
            workers: Vec::new(),
            round: 0,
        }
    }

    fn round_robin(&mut self, task:AsyncTask) {
        self.round = (self.round + 1) % self.workers.len();
        self.workers
            .get_mut(self.round)
            .unwrap()
            .task_queue
            .push(task);
    }
}


pub struct Server<P: AsyncProtocol> {
    port_mappings:HashMap<u16, Arc<P>>,
    event_manager: EventManager,
    thread_pool: ThreadPool,
}
pub(crate) type AsyncTask = Pin<Box<dyn Future<Output=()> + Send>>;
impl<P: AsyncProtocol> Server<P> {

    pub fn new() -> Self {
        Self {
            port_mappings: HashMap::new(),
            event_manager: EventManager::new(),
            thread_pool: ThreadPool::new(),
        }
    }

    pub fn listen_port(&mut self, port: u16) {
        let socket = SocketAddr::new( IpAddr::V4(Ipv4Addr::new(0,0,0,0)), port );
        let listener:TcpListener = TcpListener::bind(socket).unwrap();

        loop {
            let (stream, peer) = match listener.accept() {
                Ok((stream, peer)) => (stream, peer),
                Err(_e) => continue,
            };

            let protocol = match self.port_mappings.get(&port) {
                Some(p) => Arc::clone(p),
                None => continue,
            };

            let task:AsyncTask = Box::pin(async move {
                protocol.handle_async_connection(stream, peer).await;
            });

            self.thread_pool.round_robin(task);
        }

    }
}