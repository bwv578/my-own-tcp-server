use std::collections::HashMap;
use std::net::{SocketAddr};
use serde_json::Value;
use crate::protocols::http::method::Method;

pub struct HttpRequest {
    pub method: Method,
    pub endpoint: String,
    pub header: HashMap<String, String>,
    pub query_params: Value,
    pub body_params: Value,
    pub peer: SocketAddr
}

impl HttpRequest {
    pub fn new(method: Method, endpoint:String, header:HashMap<String, String>,
               query_params: Value, body_params: Value, peer: SocketAddr) -> Self
    {
        Self { method, endpoint, header, query_params, body_params, peer }
    }
}