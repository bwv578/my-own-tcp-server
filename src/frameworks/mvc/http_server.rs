use std::error::Error;
use std::sync::{Arc, OnceLock, RwLock, RwLockWriteGuard};
use rustls::ServerConfig;
use crate::protocols::http::http::Http;
use crate::protocols::http::http_request::HttpRequest;
use crate::protocols::http::http_response::HttpResponse;
use crate::protocols::http::method::Method;
use crate::server::server::{Port, Server};

static HTTP_MVC:OnceLock<Arc<RwLock<Http>>> = OnceLock::new();

fn get_protocol_writer() -> RwLockWriteGuard<'static, Http> {
    HTTP_MVC
        .get_or_init(|| {
            Arc::new(RwLock::new(Http::new()))
        })
        .write()
        .unwrap()
}

pub fn handle_request(
    method:Method, path:&str,
    action: fn(HttpRequest, HttpResponse) -> Result<(), Box<dyn Error>>
)
{
    get_protocol_writer()
        .handle(method, path, action);
}

pub fn start(port_nums:Vec<u16>, max_threads:u16, tls_config:Option<ServerConfig>) {
    let mut ports:Vec<Port> = Vec::new();
    for port_num in port_nums {
        ports.push(
            Port::new(port_num, HTTP_MVC.get().unwrap().clone())
        )
    }

    let mut server = Server::new(ports, max_threads);

    if let Some(tls_config) = tls_config {
        server.tls_config = Some(Arc::new(tls_config));
    }

    server.start();
}