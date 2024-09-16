pub(crate) mod router;

use binuid_shared::{
    hyper::{
        body::Incoming as IncomingBody, Request, Response,
        service::service_fn, 
        server::conn::http1
    },
    tokio::{self, net::TcpListener},
    hyper_util::rt::tokio::TokioIo
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use crate::servers::router::{router, BoxBody, full};

async fn handle(req: Request<IncomingBody>) -> Result<Response<BoxBody>, Infallible> {
    match router(req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::new(full("Hello World")))
    }
}

pub async fn http_server(address: &str) {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from_str(address).expect("No able to parse address!");
    /*

    let make_service = make_service_fn(move |_conn| {
        let service = service_fn(move |req| {
            handle(req)
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });
    
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    */

    let listener = TcpListener::bind(&addr).await.unwrap();
    //println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| handle(req));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
