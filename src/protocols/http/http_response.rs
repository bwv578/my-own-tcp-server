use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;

pub struct HttpResponse {
    stream:TcpStream,
    status:u16,
    header:HashMap<String, String>,
}

impl HttpResponse {

    pub fn new(stream:TcpStream, status:u16, header:HashMap<String, String>) -> Self {
        Self { stream, status, header }
    }

    pub fn set_status(&mut self, status:u16) -> &mut Self {
        self.status = status;
        self
    }

    fn write_status(&mut self) -> Result<(), std::io::Error> {
        let status_msg = "HTTP/1.1 ".to_string()
            + self.status.to_string().as_str()
            + "\r\n";
        Ok(self.stream.write_all(status_msg.as_bytes())?)
    }

    pub fn set_header(&mut self, key:&str, value:&str) -> &mut Self {
        self.header.insert(key.to_string(), value.to_string());
        self
    }

    fn write_header(&mut self) -> Result<(), std::io::Error> {
        for (k, v) in &self.header {
            self.stream.write_all(
                format!("{}:{}\r\n", k, v).as_bytes()
            )?
        }
        self.stream.write_all(b"\r\n")?;
        Ok(())
    }

    pub fn write(&mut self, data:&str) -> Result<(), std::io::Error> {
        self.set_header("content-length", data.len().to_string().as_str());
        self.write_status()?;
        self.write_header()?;
        Ok(self.stream.write_all(data.as_bytes())?)
    }

    pub fn write_value(&mut self, value:Value) -> Result<(), std::io::Error> {
        let value_str = value.to_string();
        self.set_header("content-length", value_str.len().to_string().as_str());
        self.write_status()?;
        self.write_header()?;
        Ok(self.stream.write_all(value_str.as_bytes())?)
    }

    pub fn write_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        self.write_status()?;
        self.set_header("content-length", file_size.to_string().as_str());
        self.write_header()?;
        self.stream.write_all(&buffer)?;

        Ok(())
    }

}