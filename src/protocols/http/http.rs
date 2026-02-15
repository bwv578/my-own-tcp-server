use std::net::TcpStream;
use crate::protocols::protocol::Protocol;

pub struct Http {

}

impl Protocol for Http {
    fn handle_connection(&self, stream: TcpStream) {
        //TcpStream::from(stream);
        todo!()
    }
}