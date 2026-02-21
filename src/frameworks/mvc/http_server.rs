use std::io::Error;
use std::sync::{Arc, OnceLock, RwLock, RwLockWriteGuard};
use crate::protocols::http::http::Http;
use crate::protocols::http::http_request::HttpRequest;
use crate::protocols::http::http_response::HttpResponse;
use crate::protocols::http::method::Method;
use crate::server::server::Server;

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
    action: fn(HttpRequest, HttpResponse) -> Result<(), Error>
)
{
    get_protocol_writer()
        .handle(method, path, action);
}

pub fn start(port:u16, max_threads:u16, use_tls:bool) {
    Server::new(
        HTTP_MVC.get().unwrap().clone(),
        port, max_threads, use_tls
    ).start();
}