use std::sync::{Arc, RwLock};
use ::server::frameworks::http_mvc::http_protocol::RW_PROT;
use ::server::protocols::http::http::Http;
use ::server::protocols::http::method::Method;
use ::server::protocols::protocol::Protocol;
use ::server::server::server::Server;

mod protocols;
mod server;

fn main() {
    //test
    static CONTENT_ROOT: &str = "./examples";


    RW_PROT.get_or_init(|| {
        Arc::new(RwLock::new(Http::new()))
    });

    {
        let rw_proc_write = &mut *RW_PROT.get().unwrap().write().unwrap();

        rw_proc_write.handle(Method::GET, "/does/rwlock/works/*", |req, mut res| {
            res.write("<h1>RW Lock works OoO</h1>")?;
            Ok(())
        });

        rw_proc_write.handle(Method::GET, "/welcome", |_req, mut res| {
            res.write_file(
                &format!("{}{}", CONTENT_ROOT, "/hello.html")
            )?;
            Ok(())
        });

        rw_proc_write.handle(Method::GET, "/img/*", |req, mut res| {
            res.write_file(
                &format!("{}{}", CONTENT_ROOT, req.endpoint)
            )?;
            Ok(())
        });

    } // 스택블록 안잡아주면 RwLock Write 변수 드랍 안됨 => 락 잡고 안놔줌 => 요청 처리시 다른데서 접근 불가

    let upcast_wrapper = Arc::clone(&*RW_PROT.get().unwrap()) as Arc<RwLock<dyn Protocol>>;
    let mut rwserver:Server = Server::new(upcast_wrapper, 60, 4);
    rwserver.start();
}
