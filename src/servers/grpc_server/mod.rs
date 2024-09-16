mod grpc_service;

use tonic::transport::{/*Certificate, Identity, */Server/*, ServerTlsConfig*/};
use crate::binuid_grpc::binuid_grpc_server::BinuidGrpcServer;
use self::grpc_service::GrpcService;



pub async fn grpc_server(address: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //let cert = tokio::fs::read(CONFIG.central_certificate.cert_path.clone()).await?;
    //let key = tokio::fs::read(CONFIG.central_certificate.key_path.clone()).await?;
    //let server_identity = Identity::from_pem(cert, key);

    //let client_ca_cert = tokio::fs::read(CONFIG.certificate.root_ca.clone()).await?;
    //let client_ca_cert = Certificate::from_pem(client_ca_cert);

    
    let server = GrpcService::new().await;

    /*let tls = ServerTlsConfig::new()
        .identity(server_identity);
        //.client_ca_root(client_ca_cert);
        */
    
    Server::builder()
        //.tls_config(tls)?
        .accept_http1(true)
        .add_service(tonic_web::enable(BinuidGrpcServer::new(server)))
        .serve(address.parse()?)
        .await?;

    Ok(())
}