use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use rustls::ServerConfig;
use crate::applications::model::Protocol;

pub struct Smtp {
    domain: String
}

impl Protocol for Smtp {
    fn handle_connection(&self, mut stream:TcpStream, peer: SocketAddr, config: Option<Arc<ServerConfig>>) -> Result<(), Box<dyn Error>> {
        let msg_ready = format!("220 {} ESMTP ready\r\n", &self.domain);
        stream.write_all(msg_ready.as_bytes())?;

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
    pub fn new(domain:&str) -> Self {
        Self {domain: domain.to_string()}
    }
}
