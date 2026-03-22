use std::io;
use std::sync::{Mutex, MutexGuard, OnceLock};
use crate::applications::async_web::http::{HttpRequest, HttpResponse, Method};
use crate::applications::async_web::protocol::Http;

static HTTP_MVC:OnceLock<Mutex<Option<Http>>> = OnceLock::new();

fn as_guard() -> MutexGuard<'static, Option<Http>> {
    HTTP_MVC.get_or_init(|| {
            Mutex::new(Some(Http::new()))
        })
        .lock().unwrap()
}

pub fn route<Fut, F>(method:Method, path:&str, action: F)
where
    Fut: Future<Output = io::Result<usize>> + Send + 'static,
    F: Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static,
{ as_guard().as_mut().unwrap().handle(method, path, action); }

pub fn extract() -> Http {
    HTTP_MVC.get().unwrap()
        .lock().unwrap()
        .take().unwrap()
}