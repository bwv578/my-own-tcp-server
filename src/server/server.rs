use std::net::TcpListener;
use std::sync::Arc;
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::{ThreadPool};

pub struct Server {
    protocol: Arc<dyn Protocol>,
    port: u16,
    thread_pool: ThreadPool,
}

pub type Task = Box<dyn FnOnce() + Send + Sync + 'static>;

impl Server {

    pub fn new(protocol: Arc<dyn Protocol>, port: u16, max_threads: u16) -> Server {
        Server {
            protocol, port,
            thread_pool: ThreadPool::new(max_threads)
        }
    }

    pub fn listen(ip:&str, port:u16) -> TcpListener {
        let listen_to = format!("{}:{}", ip, port);
        TcpListener::bind(listen_to).unwrap()
    }

    pub fn start(&mut self) {
        let listener:TcpListener = Self::listen("0.0.0.0", self.port);

        for stream in listener.incoming() {
            let protocol = Arc::clone(&self.protocol);
            let task:Task = Box::new(move || {
                protocol.handle_connection(stream.unwrap());
            });
            self.thread_pool.push_task(task);
        }

    }

}