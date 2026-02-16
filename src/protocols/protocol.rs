use std::net::TcpStream;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream: TcpStream) -> Result<(), std::io::Error>;
}