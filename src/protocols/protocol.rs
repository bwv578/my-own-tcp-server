use std::net::TcpStream;
use crate::protocols::http::request::Request;

pub trait Protocol: Send + Sync {
    fn handle_connection(&self, stream: TcpStream);
}

pub type Action = Box<dyn Fn(Request) + Send + Sync>;