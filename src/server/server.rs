use std::net::TcpListener;
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::ThreadPool;

pub struct Server {
    protocol: Box<dyn Protocol>,
    port: u16,
    thread_pool: ThreadPool,
}

impl Server {

    pub fn new(protocol: Box<dyn Protocol>, port: u16, max_threads: u16) -> Server {
        Server {
            protocol, port,
            thread_pool: ThreadPool::new(max_threads)
        }
    }

    pub fn listen(ip:&str, port:u16) -> TcpListener {
        let mut listen_to:String = String::from(ip);
        listen_to.push_str(port.to_string().as_str());
        TcpListener::bind(ip).unwrap()
    }

    pub fn start(&mut self) {
        let listener:TcpListener = Self::listen("0.0.0.0", self.port);
        for stream in listener.incoming() {

        }

    }

}