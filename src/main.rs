use std::io::Write;
use std::sync::Arc;
use crate::protocols::http::http::Http;
use crate::protocols::http::method::Method;
use crate::protocols::http::request::Request;
use crate::server::server::Server;

mod protocols;
mod server;

fn main() {
    //test
    let mut proc = Http::new();

    fn handle_hello_get(mut request:Request) {
        request.stream.write_all(b"HTTP/1.1 200 OK\r\n\
        Content-Length: 100\r\n\
        Content-Type: text/html\r\n
        \r\n\
        <h1>hi</h1>\r\n\
        <h2>you requested /hello/get</h2>\r\n\
        <h3>bye</h3>\r\n\
        ").unwrap();
    }

    fn handle_hello_post(mut request:Request) {
        request.stream.write_all(b"HTTP/1.1 200 OK\r\n\
        Content-Length: 100\r\n\
        Content-Type: text/plain\r\n
        \r\n\
        hi,\r\n\
        you requested /hello/post\r\n\
        bye.\r\n\
        ").unwrap();
    }

    proc.handle(Method::GET, "/hello/get", handle_hello_get);
    proc.handle(Method::POST, "/hello/post", handle_hello_post);

    let mut server = Server::new(Arc::new(proc), 80, 4);
    server.start();
}
