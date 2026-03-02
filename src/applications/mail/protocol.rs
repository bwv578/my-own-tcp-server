use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use rustls::ServerConfig;
use crate::applications::model::Protocol;

pub struct Smtp {
}

impl Protocol for Smtp {
    fn handle_connection(&self, stream: TcpStream, peer: SocketAddr, config: Option<Arc<ServerConfig>>) -> Result<(), Box<dyn Error>> {
        let mut line_buf = String::new();
        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut line_buf)?;

        while !line_buf.is_empty() {
            let line = std::mem::take(&mut line_buf);

            println!("line: {}", line);

            reader.read_line(&mut line_buf)?;
        }

        Ok(())
    }
}

impl Smtp {
    pub fn new() -> Self {
        Self {}
    }
}