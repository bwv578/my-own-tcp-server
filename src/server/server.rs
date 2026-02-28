use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, PoisonError, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use rustls::{ServerConnection, StreamOwned};
use crate::protocols::protocol::Protocol;
use crate::server::thread_pool::{ThreadPool};

pub struct Server {
    pub ports: Vec<Port>,
    thread_pool: ThreadPool,
    pub tls_config: Option<Arc<rustls::ServerConfig>>,
}

#[derive(Clone)]
pub struct Port {
    port_num: u16,
    protocol: Arc<RwLock<dyn Protocol>>,
}

pub type Task = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + Sync + 'static>;

pub trait ReadWrite: Read + Write + Send + Sync + 'static {}
impl<T: Read + Write + Send + Sync + 'static> ReadWrite for T {}

impl Port {
    pub fn new(port_num: u16, protocol: Arc<RwLock<dyn Protocol>>) -> Self {
        Self { port_num, protocol }
    }
}

impl Server {

    pub fn new(ports:Vec<Port>, max_threads: u16) -> Server {
        Server {
            ports,
            thread_pool: ThreadPool::new(max_threads),
            tls_config: None,
        }
    }

    fn get_listener(ip:&str, port:u16) -> TcpListener {
        let listen_to = format!("{}:{}", ip, port);
        TcpListener::bind(listen_to).unwrap()
    }

    fn log_error(){
        todo!()
    }

    pub fn start(self) {
        let server = Arc::new(self);
        let mut join_handles:Vec<JoinHandle<()>> = Vec::new();

        for port in &server.ports {
            let server_clone = server.clone();
            let port_clone = port.clone();

            let port_handle = thread::spawn(move || {
                server_clone.listen_port(port_clone);
            });

            join_handles.push(port_handle);
        }

        // block main thread
        for handle in join_handles {
            handle.join().unwrap();
        }
    }

    fn listen_port(&self, port:Port) {
        let listener:TcpListener = Self::get_listener("0.0.0.0", port.port_num);
        let protocol:Arc<RwLock<dyn Protocol>> = port.protocol.clone();

        println!("Server listening on port {}", port.port_num);

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

            let protocol_lock = Arc::clone(&protocol);
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
                        println!("Worker: {:?}", std::thread::current().id());
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