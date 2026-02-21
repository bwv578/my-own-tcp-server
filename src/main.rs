use std::error::Error;
use ::server::frameworks::mvc::*;
use ::server::protocols::http::method::Method;

mod protocols;
mod server;

fn main() {
    //test
    static CONTENT_ROOT: &str = "./examples";

    http_server::handle_request(
        Method::GET, "/does/rwlock/works/*", |req, mut res| {
        res.write("<h1>RW Lock works OoO</h1>")?;
        Ok(())
    });

    http_server::handle_request(
        Method::GET, "/welcome", |_req, mut res| {
            let mut i:usize = 0;
            i -= 500;
            println!("Welcome {}!", i);
            res.write_file(
                &format!("{}{}", CONTENT_ROOT, "/hello.html")
            )?;
            Ok(())
        }
    );

    http_server::handle_request(
        Method::GET, "/img/*", |req, mut res| {
        res.write_file(
            &format!("{}{}", CONTENT_ROOT, req.endpoint)
        )?;
        Ok(())
    });

    http_server::start(8080, 3, false);

}
