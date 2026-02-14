use std::net::TcpStream;

pub trait Protocol {
    fn handle_connection(&mut self, stream: TcpStream);
}