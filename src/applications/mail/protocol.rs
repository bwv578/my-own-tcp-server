use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use rustls::ServerConfig;
use crate::applications::model::Protocol;

pub struct Smtp {
}

impl Protocol for Smtp {
    fn handle_connection(&self, stream: TcpStream, peer: SocketAddr, config: Option<Arc<ServerConfig>>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}