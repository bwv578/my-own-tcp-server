use std::collections::{HashMap};
use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::TcpStream;
use serde_json::Value;
use crate::protocols::http::handler::{Handler};
use crate::protocols::http::method::Method;
use crate::protocols::http::request::Request;
use crate::protocols::http::util::decode_query;
use crate::protocols::protocol::{Action, Protocol};

pub struct Http {
    handlers:HashMap<(Method, String), Handler>
}

impl Protocol for Http {
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut query_params:Value = Value::Object(serde_json::Map::new());

        let request_line:(Method, String) = match Self::parse_request_line(&mut buf_reader, &mut query_params) {
            Ok (request) => request,
            Err(_error) => {
                stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\
                Content-Length: 11\r\n\
                Content-Type: text/plain\r\n"
                ).unwrap();
                return;
            }
        };

        let header:HashMap<String, String> = Self::parse_header(&mut buf_reader);

        let content_length:usize = match header.get("Content-Length") {
            Some(content_length) => {
                content_length.parse::<usize>().unwrap_or(0)
            }
            _ => 0
        };

        let content_type:String = match header.get("Content-Type") {
            Some(content_type) => content_type.clone(),
            _ => String::from("text/plain")
        };

        let body_params:Value = match Self::parse_body(&mut buf_reader, content_length, content_type) {
            Ok(Some(body)) => body,
            Err(_error) => {
                stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\
                Content-Length: 11\r\n\
                Content-Type: text/plain\r\n"
                ).unwrap();
                return;
            },
            _ => Value::Null
        };

        // check
        println!("request: {:?}, {}", request_line.0, request_line.1);
        println!("query params: {:?}", query_params);
        println!("header: {:?}", header);
        println!("body params: {:?}", body_params);

        match self.handlers.get(&request_line) {
            Some(handler) => {
                handler.execute(
                    Request::new(
                        request_line.0, request_line.1, header,
                        body_params, query_params, stream
                    )
                );
            },
            _ => { // 핸들러 없음 => 404
                stream.write_all(b"HTTP/1.1 404 Not Found\r\n\
                Content-Length: 10\r\n\
                Content-Type: text/plain\r\n\
                \r\n\
                Not Found").unwrap();
                return;
            }
        }
    }
}

impl Http {
    pub fn new() -> Self {
        Self{ handlers: HashMap::new() }
    }

    fn parse_request_line(reader:&mut BufReader<&mut TcpStream>, params:&mut Value)
        -> Result<(Method, String), Error>
    {
        let mut line:String = String::new();
        reader.read_line(&mut line)?;

        let request:Vec<&str> = line.trim().split(' ').collect();
        let method:Method = match request[0].trim().to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "CONNECT" => Method::CONNECT,
            _ => {
                return Err(Error::new(
                    io::ErrorKind::InvalidData,
                    "unknown method"
                ));
            }
        };

        let queries:Vec<&str> = request[1].trim().split("?").collect();
        let endpoint = queries.get(0).unwrap_or(&"");
        let query_string = queries.get(1).unwrap_or(&"");
        decode_query(query_string, params);

        Ok((method, endpoint.to_string()))
    }

    fn parse_header(reader:&mut BufReader<&mut TcpStream>) -> HashMap<String, String> {
        let mut header:HashMap<String, String> = HashMap::new();
        let mut line_buf = String::new();
        loop {
            line_buf.clear();
            let n = reader.read_line(&mut line_buf).unwrap();
            if n == 0 || line_buf.trim().is_empty() { break; }

            let kv: Vec<String> = line_buf.trim()
                .split(':')
                .map(|s| s.to_string())
                .collect();

            let key = kv.get(0).unwrap_or(&String::new()).trim().to_string();
            let value = kv.get(1).unwrap_or(&String::new()).trim().to_string();
            header.insert(key, value);
        }
        return header;
    }

    fn parse_body(reader:&mut BufReader<&mut TcpStream>, content_length:usize, content_type:String)
        -> Result<Option<Value>, Error>
    {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body)?;
        let body_str = String::from_utf8(body).unwrap();

        match content_type.as_str() {
            "text/plain" => Ok(Some(Value::String(body_str))),
            "application/x-www-form-urlencoded" => Ok(None),
            "multipart/form-data" => Ok(None), // todo() 멀티파트 처리
            "application/json" => {
                Ok(Some(serde_json::from_str(body_str.as_str())?))
            },
            _ => Err(Error::new(
                io::ErrorKind::InvalidData,
                "unknown content-type"
            ))
        }
    }

    pub fn handle(&mut self, method: Method, endpoint:&str, action: fn(Request)) {
        self.handlers.insert(
            (method.clone(), endpoint.to_string().clone()),
            Handler::new(method, &endpoint, Box::new(action))
        );
    }

}