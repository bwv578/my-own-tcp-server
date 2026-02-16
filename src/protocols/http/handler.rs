use std::io::Error;
use crate::protocols::http::http::HttpAction;
use crate::protocols::http::method::Method;
use crate::protocols::http::http_request::{HttpRequest};
use crate::protocols::http::http_response::HttpResponse;

pub struct Handler {
    method: Method,
    endpoint: String,
    action: HttpAction
}

impl Handler {

    pub fn new(method: Method, endpoint:&str, action: HttpAction) -> Self {
        Self { method, endpoint:String::from(endpoint), action }
    }

    pub fn execute(&self, request:HttpRequest, response:HttpResponse) -> Result<(), Error> {
        Ok((*self.action)(request, response)?)
    }

}