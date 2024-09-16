use binuid_shared::hyper::{body::Incoming as IncomingBody, Request, Response, StatusCode};
use binuid_shared_wasm::serde_json::json;
use super::{BoxBody, full, Params};

pub(crate) async fn unknowed_route(_req: Request<IncomingBody>, _params: Params) -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(full(json!({"error": "Route Not Found!"}).to_string()))
        .unwrap()
}