use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

use crate::utils::api::models;
use crate::{config, utils};

pub async fn start(project: &models::Project) {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 4011));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
async fn handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("[PROXY]: {:?}", _req.body());
    let route = String::from("/wasm-hello-world");
    let uri = _req.uri();
    let config = config::project::read_project_connection().expect("No connecte project");
    if uri.path().eq(&route) {
        let wasm_res = utils::wasm::run(&config.path).unwrap();
        let json_str = serde_json::to_string(&wasm_res).unwrap();
        Ok(Response::new(json_str.into()))
    } else {
        Ok(Response::new("".into()))
    }
}
