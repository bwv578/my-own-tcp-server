use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::{ThreadPool};

pub struct Server {
    //protocol: Arc<dyn Protocol>,
    protocol: Arc<RwLock<dyn Protocol>>,
    port: u16,
    thread_pool: ThreadPool,
}

pub type Task = Box<dyn FnOnce() + Send + Sync + 'static>;

impl Server {

    pub fn new(protocol:Arc<RwLock<dyn Protocol>>, port: u16, max_threads: u16) -> Server {
        Server {
            protocol, port,
            thread_pool: ThreadPool::new(max_threads)
        }
    }

    pub fn listen(ip:&str, port:u16) -> TcpListener {
        let listen_to = format!("{}:{}", ip, port);
        TcpListener::bind(listen_to).unwrap()
    }

    fn log_error(){
        todo!()
    }

    pub fn start(&mut self) {
        let listener:TcpListener = Self::listen("0.0.0.0", self.port);
        println!("Server listening on port {}", self.port);

        for stream in listener.incoming() {
            let protocol = Arc::clone(&self.protocol);
            let task:Task = Box::new(move || {
                let read = protocol.read().unwrap();
                match read.handle_connection(stream.unwrap())
                {
                    Ok(_) => {}, // todo log_connection ?
                    Err(e) => {
                        println!("Error handling connection: {:?}", e);
                        //Self::log_error();
                    }
                }
            });
            self.thread_pool.push_task(task);
        }
    }

}