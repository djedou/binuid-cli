use crate::Result;
use hyper::{Uri, client::conn::http1, body::{Bytes, Buf}, Request};
use hyper_util::rt::tokio::TokioIo;
use tokio::{task, net::{TcpListener, TcpStream}};
use http_body_util::{BodyExt, combinators::BoxBody};
use serde::{Deserialize, Serialize};
use crate::read_binuid_config;


pub(crate) async fn publish() -> Result<()> {
    println!("publishing...");
    let config = read_binuid_config("")?;
    match (config.library.as_ref(), config.binary.as_ref(), config.workspace.as_ref()) {
        (Some(library), _, _) => {
            let package = &library.name;
            let author = library.authors.get(0).expect("No author is provided!").split(" ").collect::<Vec<_>>();
            let name = author[0];
            let email = author[1];
            let version = library.version.as_ref().expect("No version is provided!");

            let url = format!("http://127.0.0.1:4012/repositories/create_package_git/workspace/{package}/{version}").parse().unwrap();
            let package_git = create_package_git(url, vec![User {name: name.to_string() , email: email.to_string() }]).await?;
            match package_git.state.as_str() {
                "Ready" => {
                    println!("Ready");


                    
                },
                a => {
                    println!("{a}");
                }
            }

            Ok(())
        },
        (_, _, _) => {
            Ok(())
        }
    }
}

async fn create_package_git(url: Uri, users: Vec<User>) -> Result<PackageGitState> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = http1::handshake(io).await?;
    task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    //let authority = url.authority().unwrap().clone();

    // Fetch the url...
    let req = Request::post(url)
        //.header(hyper::header::HOST, authority.as_str())
        .body(BoxBody::new(serde_json::json!(users).to_string()))?;

    let res = sender.send_request(req).await?;

    // asynchronously aggregate the chunks of the body
    let body = res.collect().await?.aggregate();
    // try to parse as json with serde_json
    let res = serde_json::from_reader(body.reader())?;
    Ok(res)
}

#[derive(Deserialize, Debug, Serialize)]
struct PackageInfos {
    id: i32,
    #[allow(unused)]
    name: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct PackageGitState {
    state: String,
}


#[derive(Deserialize, Debug, Serialize)]
struct User {
    name: String,
    email: String
}