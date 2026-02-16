#[derive(Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub enum Method {
    GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH, CONNECT, TRACE, ANY
}