use std::sync::Arc;
use serde::Deserializer;
use serde_json::Value;
use crate::protocols::http::http::Http;
use crate::protocols::http::method::Method;
use crate::protocols::http::http_request::{HttpRequest};
use crate::protocols::http::http_response::HttpResponse;
use crate::server::server::Server;

mod protocols;
mod server;

fn main() {
    //test
    let mut proc = Http::new();

    fn handle_hello_get(request:HttpRequest, mut response:HttpResponse) -> Result<(),std::io::Error> {
        response.write("HTTP/1.1 200 OK\r\n\
        Content-Length: 100\r\n\
        Content-Type: text/html\r\n
        \r\n\
        <h1>hi</h1>\r\n\
        <h2>you requested /hello/get</h2>\r\n\
        <h3>bye</h3>\r\n\
        ")?;

        Ok(())
    }

    fn handle_hello_post(request:HttpRequest, mut response:HttpResponse) -> Result<(), std::io::Error> {
        response.write("HTTP/1.1 200 OK\r\n\
        Content-Length: 100\r\n\
        Content-Type: text/plain\r\n
        \r\n\
        hi,\r\n\
        you requested /hello/post\r\n\
        bye.\r\n\
        ")?;

        Ok(())
    }

    fn handle_test1_get(request: HttpRequest, mut response: HttpResponse) -> Result<(), std::io::Error> {
        let body = format!("<h1>hi hello, {} : {}</h1>
        <h2>you requested /test/get/hello</h2>
        <h3>your parameter: {}</h3>
        ",
                           request.peer.ip(),
                           request.peer.port(),
                           serde_json::to_string(&request.query_params).unwrap_or(String::from(""))
        );

        response
            .set_status(200)
            .set_header("Content-Type", "text/html")
            .write(body.as_str())?;

        Ok(())
    }

    fn handle_test1_post(request: HttpRequest, mut response: HttpResponse) -> Result<(), std::io::Error> {
        let body = format!(
            "{{ \
            \"peer-ip\": \"{}\", \
            \"peer-port\": {}, \
            \"your-params\": \"{}\" \
            }}",
            request.peer.ip(),
            request.peer.port(),
            serde_json::to_string(&request.body_params).unwrap_or(String::from(""))
        );

        response
            .set_status(200)
            .set_header("Content-Type", "application/json")
            .write(body.as_str())?;

        Ok(())
    }


    proc.handle(Method::GET, "/hello/get", handle_hello_get);
    proc.handle(Method::POST, "/hello/post", handle_hello_post);
    proc.handle(Method::GET, "/test/get/hello", handle_test1_get);
    proc.handle(Method::POST, "/test/post/hello", handle_test1_post);

    let mut server = Server::new(Arc::new(proc), 80, 4);
    server.start();
}
