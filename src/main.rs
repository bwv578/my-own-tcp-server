use std::fs::File;
use std::io::BufReader;
use rustls::ServerConfig;
use ::server::frameworks::mvc::*;
use ::server::protocols::http::method::Method;
use rustls_pemfile::{certs, pkcs8_private_keys};

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

    let cert_file = &mut BufReader::new(File::open("./cert/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("./cert/key.pem").unwrap());
    let certs = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let key = pkcs8_private_keys(key_file)
        .next().unwrap().unwrap();

    let tls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key.into())
        .unwrap();

    //http_server::start(443, 3, Some(tls_config));

    http_server::start(vec![7070, 8080, 8081, 8082], 2, None);
    
}
