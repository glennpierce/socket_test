use pretty_env_logger;

use hyper;
use hyper::{Url, Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};



static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone, Copy)]
struct Echo;

#[derive(Clone, Copy)]
struct Sensor;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        ::futures::finished(match (req.method(), req.path()) {
            (&Get, "/") | (&Get, "/echo") => {

                //let mut path_segments = url.path_segments().unwrap();

                let query = req.uri();
                //let url = Url::parse(&query.to_string()).unwrap();
                println!("{}", query);

                Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)
            }
            (&Post, "/echo") => {
                let mut res = Response::new();
                if let Some(len) = req.headers().get::<ContentLength>() {
                    res.headers_mut().set(len.clone());
                }
                res.with_body(req.body())
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

pub fn serve() {
    //pretty_env_logger::init().unwrap();
    let addr = "127.0.0.1:1337".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    println!("Listening on http://{}", server.local_addr().unwrap());
    server.run().unwrap();
}
