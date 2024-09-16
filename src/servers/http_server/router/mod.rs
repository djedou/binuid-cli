//mod user_routes;
pub(crate) mod unknowned;
pub(crate) mod response;
pub(crate) mod init_route;
pub(crate) mod engine_routes;
pub(crate) mod match_request;

pub(crate) use response::full;
pub(crate) use match_request::*;

use binuid_shared::{
    hyper::{self, body::Incoming as IncomingBody, Request, Response},
    futures::future::{BoxFuture},
    http_body_util,
    bytes::Bytes
};

use self::{
    unknowned::unknowed_route,
    init_route::init_route,
    engine_routes::{binuid_engine_js, binuid_engine_wasm, binuid_redirect_to_root}
};


// A boxed type definition for your async views.
pub type RouterHandler = Box<dyn Fn(Request<IncomingBody>, Params) -> BoxFuture<'static, Response<BoxBody>> + Send + Sync>;
pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[macro_export]
macro_rules! route_handler {
    ($closure:expr) => {{
        #[allow(unused_mut)]
        let mut closure = $closure;
        let b: crate::servers::router::RouterHandler
         = Box::new(move |req, params| {
            Box::pin(closure(req, params))
        });
        b
    }};
}

// An example request router.
pub async fn router(req: Request<IncomingBody>) -> Result<Response<BoxBody>, Error> {
    let method = req.method();
    let path = req.uri().path();

    let (handler, params) = crate::match_request!(method, path, {
        "/" => {
            GET => crate::route_handler!(init_route), 
            POST => crate::route_handler!(unknowed_route), 
        },
        "/binuid_engine" => {
            GET => crate::route_handler!(binuid_engine_js), 
            POST => crate::route_handler!(unknowed_route), 
        },
        "/favicon.ico" => {
            POST => crate::route_handler!(unknowed_route), 
            GET => crate::route_handler!(unknowed_route), 
        },
        "/binuid_engine_bg.wasm" => {
            GET => crate::route_handler!(binuid_engine_wasm),
            POST => crate::route_handler!(unknowed_route),
        },
        "/*" => {
            POST => crate::route_handler!(binuid_redirect_to_root),
            GET => crate::route_handler!(binuid_redirect_to_root),
        }
    }).unwrap();

    Ok(handler(req, params).await)
}