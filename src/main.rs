use std::fmt::format;
use std::sync::Arc;
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
    static CONTENT_ROOT: &str = "./examples";

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

    fn file_test_get(request: HttpRequest, mut response: HttpResponse) -> Result<(),std::io::Error> {
        println!("try file {}", &format!("{}hello.html", CONTENT_ROOT));
        Ok(response.write_file(
            &format!("{}/hello.html", CONTENT_ROOT)
        )?)
    }

    fn img_test_get(request: HttpRequest, mut response: HttpResponse) -> Result<(),std::io::Error> {
        Ok(response.write_file(
            &format!("{}/test.jpg", CONTENT_ROOT)
        )?)
    }

    fn wildcard_test_img(request: HttpRequest, mut response: HttpResponse) -> Result<(),std::io::Error> {
        print!("random image requested! : {}", &format!("{}{}", CONTENT_ROOT, request.endpoint));
        Ok((response.write_file(
            &format!("{}{}", CONTENT_ROOT, request.endpoint)
        )?))
    }

    fn wildcard_test_file(request: HttpRequest, mut response: HttpResponse) -> Result<(),std::io::Error> {
        Ok((response.write_file(
            &format!("{}{}", CONTENT_ROOT, request.endpoint)
        )?))
    }

    proc.handle(Method::GET, "/hello/get", handle_hello_get);
    proc.handle(Method::POST, "/hello/post", handle_hello_post);
    proc.handle(Method::GET, "/test/get/hello", handle_test1_get);
    proc.handle(Method::POST, "/test/post/hello", handle_test1_post);

    // file & image test
    proc.handle(Method::GET, "/test/file/hello", file_test_get);
    proc.handle(Method::GET, "/img/test.jpg", img_test_get);

    // wildcard test
    proc.handle(Method::GET, "/img/any/*", wildcard_test_img);
    proc.handle(Method::GET, "/files/any/*", wildcard_test_file);

    let mut server = Server::new(Arc::new(proc), 80, 4);
    server.start();

}
