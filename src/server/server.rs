use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, PoisonError, RwLock};
use std::time::Duration;
use rustls::{ServerConnection, StreamOwned};
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::{ThreadPool};

pub struct Server {
    protocol: Arc<RwLock<dyn Protocol>>,
    port: u16,
    thread_pool: ThreadPool,
    tls_config: Option<Arc<rustls::ServerConfig>>,
}

pub type Task = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + Sync + 'static>;
pub trait ReadWrite: Read + Write + Send + Sync + 'static {}
//impl ReadWrite for StreamOwned<ServerConnection, TcpStream> {}
//impl ReadWrite for TcpStream {}
impl<T: Read + Write + Send + Sync + 'static> ReadWrite for T {}

impl Server {

    pub fn new(protocol:Arc<RwLock<dyn Protocol>>, port: u16, max_threads: u16) -> Server {
        Server {
            protocol, port,
            thread_pool: ThreadPool::new(max_threads),
            tls_config: None,
        }
    }

    pub fn listen(ip:&str, port:u16) -> TcpListener {
        let listen_to = format!("{}:{}", ip, port);
        TcpListener::bind(listen_to).unwrap()
    }

    fn log_error(){
        todo!()
    }

    pub fn tls_config(&mut self, config:Arc<rustls::ServerConfig>){
        self.tls_config = Some(config);
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

            let peer = match stream.peer_addr() {
                Ok(peer) => peer,
                Err(_e) => continue // TODO LOG ERROR
            };

            match stream.set_read_timeout(Some(Duration::from_secs(3))) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error handling connection: {:?}", e);
                    // TODO LOG ERROR
                    continue
                }
            }

            let stream_to_handle:Box<dyn ReadWrite> = match self.tls_config {
                Some(ref config) => {
                    let conn = ServerConnection::new(config.clone()).unwrap();
                    let tls_stream = StreamOwned::new(conn, stream);
                    Box::new(tls_stream)
                },
                None => Box::new(stream)
            };

            let protocol_lock = Arc::clone(&self.protocol);
            let task:Task = Box::new(move || {
                let protocol = match protocol_lock.read() {
                    Ok(read) => read,
                    Err(e) => {
                        return Err(Box::new(PoisonError::new(e.to_string())));
                    } // TODO LOG ERROR
                };
                match protocol.handle_connection(stream_to_handle, peer) {
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