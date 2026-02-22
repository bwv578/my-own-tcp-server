use std::error::Error;
use std::net::SocketAddr;
use crate::server::server::ReadWrite;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream:Box<dyn ReadWrite>, peer:SocketAddr) -> Result<(), Box<dyn Error>>;
}