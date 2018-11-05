extern crate hyper;
extern crate futures;
extern crate url;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_json;

use hyper::{StatusCode};
use hyper::Method::{Get};
use hyper::server::{Request, Response, Service};
use hyper::header::{ContentLength, ContentType};

//use futures::Stream;
use futures::future::{Future, FutureResult};

mod blockchain;
use blockchain::{Blockchain, init};


struct Microservice;

fn make_error_response(error_message: &str) -> FutureResult<hyper::Response, hyper::Error> {
    let payload = json!({
        "error": error_message
    }).to_string();
    let response = Response::new()
        .with_status(StatusCode::InternalServerError)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    debug!("{:?}", response);
    futures::future::ok(response)
}


fn handle_404() -> FutureResult<hyper::Response, hyper::Error> {
    let payload = json!({
        "error": "Route not found"
    }).to_string();
    let response = Response::new()
        .with_status(StatusCode::NotFound)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    futures::future::ok(response)
}


impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, request: Request) -> Self::Future {
        match (request.method(), request.path()) {
            (&Get, "/") => {
                info!("Microservice received a request: {:?}", request);
                Box::new(futures::future::ok(Response::new()))
            }
            _ => {
                info!("Route not found: {:?}", request.path());
                let response = handle_404();
                Box::new(response)
            }
        }
    }
}


fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, move || Ok(Microservice))
        .unwrap();
    let mut chain: Blockchain = init();
    chain.new_block(String::from("1"), String::from("NULL"));
    info!("Blockchain: {:?}", chain);
    info!("Running microservice at {}", address);
    info!("~~Bockchain service successfully started~~");
    server.run().unwrap();
}
