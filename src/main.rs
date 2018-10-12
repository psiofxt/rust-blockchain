extern crate hyper;
extern crate futures;
extern crate url;

#[macro_use]
extern crate log;
extern crate env_logger;

//use hyper::{StatusCode};
//use hyper::Method::{Get, Post};
use hyper::server::{Request, Response, Service};
//use hyper::header::{ContentLength, ContentType};

//use futures::Stream;
use futures::future::{Future};

mod blockchain;
use blockchain::{Blockchain, init};


struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, request: Request) -> Self::Future {
        info!("Microservice received a request: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
    }
}


fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, move || Ok(Microservice))
        .unwrap();
    let chain: Blockchain = init();
    info!("Blockchain: {:?}", chain);
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
