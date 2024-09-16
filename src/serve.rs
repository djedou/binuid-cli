use crate::servers::http_server;
use binuid_shared::tokio;
use binuid_shared_wasm::tracing::info;

pub(crate) async fn serve(host: &str, port: u16) {
    println!("host: {host}");
    let http_address = format!("{host}:{port}");
    let grpc_address = format!("0.0.0.0:3002");
    info!("Http Server: http://{}", http_address);
    info!("Grpc Server: http://{}", grpc_address);
    let http_server_handle = tokio::spawn(async move {
        http_server(&http_address).await
    });

    let _ = http_server_handle.await.unwrap();
}