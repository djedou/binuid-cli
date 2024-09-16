use binuid_shared::hyper::{body::Incoming as IncomingBody, Request, Response};
use crate::servers::router::response::{ok, ok_js, ok_wasm};
use std::fs;
use super::{BoxBody, full, Params};

pub(crate) async fn binuid_engine_js(_req: Request<IncomingBody>, _params: Params) -> Response<BoxBody> {
    match read_file_vec("./pkg/binuid_engine.js") {
        Ok(res) => {
            ok_js(full(res))
        },
        Err(e) => {
            ok(full(format!("{e}")))
        }
    }
}

pub(crate) async fn binuid_engine_wasm(_req: Request<IncomingBody>, _params: Params) -> Response<BoxBody> {
    match read_file_vec("./pkg/binuid_engine_bg.wasm") {
        Ok(res) => {
            ok_wasm(full(res))
        },
        Err(e) => {
            ok(full(format!("{e}")))
        }
    }
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

pub(crate) async fn binuid_redirect_to_root(_req: Request<IncomingBody>, _params: Params) -> Response<BoxBody> {
    Response::new(full(
        r#"
        <script>
            window.location.href = "/";  
        </script>
    "#))
}
