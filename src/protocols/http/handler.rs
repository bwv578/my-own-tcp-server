use crate::protocols::http::method::Method;
use crate::protocols::protocol::Action;

pub struct Handler {
    method: Method,
    endpoint: String,
    action: Action
}

impl Handler {

    pub fn new(method: Method, endpoint:&str, action: Action) -> Self {
        Self { method, endpoint:String::from(endpoint), action }
    }

    pub fn execute(&mut self) {
        (*self.action)();
    }

}