use std::error::Error;
use std::net::TcpStream;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn Error>>;
}