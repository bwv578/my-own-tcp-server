use std::error::Error;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::sync::{Arc, PoisonError, RwLock};
use std::time::Duration;
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::{ThreadPool};

pub struct Server {
    protocol: Arc<RwLock<dyn Protocol>>,
    port: u16,
    thread_pool: ThreadPool,
    use_tls: bool,
}

pub type Task = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + Sync + 'static>;

impl Server {

    pub fn new(protocol:Arc<RwLock<dyn Protocol>>, port: u16, max_threads: u16, use_tls:bool) -> Server {
        Server {
            protocol, port,
            thread_pool: ThreadPool::new(max_threads),
            use_tls
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

        for stream_result in listener.incoming() {

            let stream = match stream_result {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Error handling connection: {:?}", e);
                    // TODO LOG ERROR
                    continue
                }
            };

            match stream.set_read_timeout(Some(Duration::from_secs(3))) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error handling connection: {:?}", e);
                    // TODO LOG ERROR
                    continue
                }
            }

            let protocol_lock = Arc::clone(&self.protocol);
            let task:Task = Box::new(move || {
                let protocol = match protocol_lock.read() {
                    Ok(read) => read,
                    Err(e) => {
                        return Err(Box::new(PoisonError::new(e.to_string())));
                    } // TODO LOG ERROR
                };
                match protocol.handle_connection(stream) {
                    Ok(result) => { Ok(result) }, // todo log_connection ?
                    Err(e) => {
                        println!("Error handling connection: {:?}", e);
                        // TODO LOG ERROR
                        Err(e)
                    }
                }
            });

            match self.thread_pool.push_task(task) {
                Ok(_) => {},
                Err(e) => {
                    // TODO LOG ERROR
                    println!("Error handling thread pool: {:?}", e);
                }
            }
        }
    }

}