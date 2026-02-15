use std::net::TcpStream;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream: TcpStream);
}

pub type Action = Box<dyn Fn() + Send + Sync>;