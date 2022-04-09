use std::convert::Infallible;
use std::net::{SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
mod webres;

pub fn run_server(ip_add:[u8; 4], port: u16) {

    async fn wer_res(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(webres::r1().into()))
    }
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {



            // We'll bind to 127.0.0.1:<port>
            let addr = SocketAddr::from((ip_add, port));

            // A `Service` is needed for every connection, so this
            // creates one from our `wer_res` function.
            let make_svc = make_service_fn(|_conn| async {
                // service_fn converts our function into a `Service`
                Ok::<_, Infallible>(service_fn(wer_res))
            });

            let server = Server::bind(&addr).serve(make_svc);

            // Run this server for... forever!
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        })
}