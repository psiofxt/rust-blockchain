//mod blockchain;

#[macro_use]
extern crate tower_web;
extern crate tokio;

//#[macro_use]
//extern crate serde_json;

use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct JsonResource;

#[derive(Debug, Response)]
struct Response {
    message: &'static str
}

#[derive(Debug, Response)]
#[web(status = "201")]
struct CreatedResponse {
    message: &'static str,

    #[web(header)]
    x_my_header: &'static str,
}

impl_web! {
    impl JsonResource {

        #[get("/")]
        #[content_type("application/json")]
        fn stats(&self) -> Result<Response, ()> {
            // TODO: display current stats
            Ok(Response {
                message: "blockchain info"
            })
        }

        #[post("/nodes/register")]
        #[content_type("application/json")]
        fn register_node(&self) -> Result<Response, ()> {
            // TODO: register a new node
            Ok(Response {
                message: "node registered!!"
            })
        }

        #[post("/mine")]
        #[content_type("application/json")]
        fn mine(&self) -> Result<CreatedResponse, ()> {
            // TODO: impl mining method + proof
            Ok(CreatedResponse {
                message: "mined a block",
                x_my_header: "awesome",
            })
        }

        #[get("/nodes/resolve")]
        #[content_type("application/json")]
        fn resolve_nodes(&self) -> Result<Response, ()> {
            // TODO: resolve longest chain
            Ok(Response {
                message: "nodes resolved!"
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(JsonResource)
        .run(&addr)
        .unwrap();
}
