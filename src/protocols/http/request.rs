use std::collections::HashMap;
use std::net::TcpStream;
use serde_json::Value;
use crate::protocols::http::method::Method;

pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub header: HashMap<String, String>,
    pub query_params: Value,
    pub body_params: Value,
    pub stream: TcpStream,
}

impl Request {
    pub fn new(method: Method, endpoint:String, header:HashMap<String, String>,
               query_params: Value, body_params: Value, stream:TcpStream) -> Self
    {
        Self { method, endpoint, header, query_params, body_params, stream }
    }
}