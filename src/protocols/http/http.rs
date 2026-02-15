use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use crate::protocols::http::handler::{Handler};
use crate::protocols::http::method::Method;
use crate::protocols::protocol::{Action, Protocol};

pub struct Http {
    handlers:HashMap<(Method, String), Handler>
}

impl Protocol for Http {
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut line_buf = String::new();

        let mut header:HashMap<String, String> = HashMap::new();

        // 메소드, 엔드포인트 + 파라미터(GET)
        buf_reader.read_line(&mut line_buf).unwrap();
        let meta: Vec<String> = line_buf
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        header.insert(String::from("method"), meta[0].trim().to_string());
        header.insert(String::from("endpoint"), meta[1].trim().to_string());

        // header
        loop {
            line_buf.clear();
            let n = buf_reader.read_line(&mut line_buf).unwrap();
            if n == 0 || line_buf.trim().is_empty() { break; }

            let kv: Vec<String> = line_buf.trim()
                .split(':')
                .map(|s| s.to_string())
                .collect();

            let key = kv.get(0).unwrap_or(&String::new()).clone();
            let value = kv.get(1).unwrap_or(&String::new()).clone();

            header.insert(key, value);
        }

        // content-length 체크
        let content_length_str = header.get("Content-Length");
        let method = header.get("method").unwrap();
        if content_length_str.is_none()
            && (method=="POST" || method=="PUT" || method=="PATCH")
        {
            stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\
            Content-Length: 11\r\n\
            Content-Type: text/plain\r\n"
            ).unwrap();
            stream.flush().unwrap();
            return;
        }

        // body
        let content_length:usize = match content_length_str{
            Some(length) => length.trim().parse().unwrap_or(0),
            None => 0
        };
        println!("Content-Length: {}", content_length);
        let mut body = vec![0; content_length];
        buf_reader.read_exact(&mut body).unwrap();

        println!("header: {:#?}", header);

        let body_str = String::from_utf8(body).unwrap();
        println!("Body: {}", body_str);


        let response:String = match method.as_str() {
            "POST" => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}",
                "{ \"data\" : \"hello POST\"}"
            ),
            "GET" => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                "<h1>hello GET</h1>",
            ),
            _ => "molla".to_string(),
        };

        stream.write(response.as_bytes()).unwrap();
    }
}

impl Http {
    pub fn new() -> Self {
        Self{ handlers: HashMap::new() }
    }

    pub fn handle(&mut self, method: Method, endpoint:String, action: Action) {
        self.handlers.insert(
            (method.clone(), endpoint.clone()),
            Handler::new(method, &endpoint, action)
        );
    }

}