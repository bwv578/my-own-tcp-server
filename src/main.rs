use std::collections::HashMap;
use std::sync::Arc;
use crate::protocols::http::http::Http;
use crate::server::server::Server;

mod protocols;
mod server;

fn main() {
    //test
    let mut server = Server::new(Arc::new(Http::new()), 80, 4);
    server.start();
}
