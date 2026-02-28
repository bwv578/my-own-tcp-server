use std::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream:TcpStream, peer:SocketAddr, config:Option<Arc<rustls::ServerConfig>>)
        -> Result<(), Box<dyn Error>>;
}

pub trait ReadWrite: Read + Write + Send + Sync + 'static {}
impl<T: Read + Write + Send + Sync + 'static> ReadWrite for T {}