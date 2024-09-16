use crate::Result;
//use hyper::{Uri, client::conn::http1, body::{Bytes, Buf}, Request};
/*
use hyper_util::rt::tokio::TokioIo;
use tokio::{task, net::{TcpListener, TcpStream}};
use http_body_util::{BodyExt, combinators::BoxBody};
use serde::{Deserialize, Serialize};
use crate::read_binuid_config;
use git2::{Repository, BranchType, Error as E, Signature, Oid};
use std::path::Path;
use std::{fs, env};
*/

pub(crate) async fn publish() -> Result<()> {
    println!("publishing...");
    /*let config = read_binuid_config("")?;
    match (config.library.as_ref(), config.binary.as_ref(), config.workspace.as_ref()) {
        (Some(library), _, _) => {
            let package = &library.name;
            let author = library.authors.get(0).expect("No author is provided!").split(" ").collect::<Vec<_>>();
            let name = author[0];
            let email = author[1].trim_start_matches("<").trim_end_matches(">");
            let version = library.version.as_ref().expect("No version is provided!");

            let url = format!("http://127.0.0.1:4012/repositories/create_package_git/workspace/{package}").parse().unwrap();
            let package_git = create_remote_package_git(url, vec![User {name: name.to_string() , email: email.to_string() }]).await?;
            match package_git.state.as_str() {
                "Ready" => {
                    println!("Ready");
                    let _ = create_local_package_git("workspace", &version, &name, &email).await;

                    
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
    }*/

    Ok(())
}
/*
async fn create_remote_package_git(url: Uri, users: Vec<User>) -> Result<PackageGitState> {
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
    // Fetch the url...
    let req = Request::post(url)
        //.header(hyper::header::HOST, authority.as_str())
        .body(BoxBody::new(serde_json::json!(users).to_string()))?;

    let res = sender.send_request(req).await?;

    // asynchronously aggregate the chunks of the body
    let body = res.collect().await?.aggregate();
    // try to parse as json with serde_json
    let res: PackageGitState = serde_json::from_reader(body.reader())?;
    Ok(res)
}

async fn create_local_package_git(workspace: &str, version: &str, name: &str, email: &str) {
    match workspace {
        "workspace" => {
            let Ok(mut current_dir) = env::current_dir() else {
                return;
            };
            match repository(&current_dir, &version, &name, &email) {
                Ok(res) => {},
                Err(e) => {}
            }
        },
        ws  => {
            /*let Ok(mut current_dir) = env::current_dir() else {
                return;
            };
            current_dir.push("crates_repositories");
            current_dir.push(ws);
            current_dir.push(package);
            match repository(&current_dir, &version, &name, &email) {
                Ok(res) => ok(BoxedBody::new(json!({"state": res}).to_string())),
                Err(e) => err(&format!("{e:#?}"))
            }*/
        }
    }
}

fn repository(path: &Path, version: &str, name: &str, email: &str) -> std::result::Result<(), E> {
    match Repository::open(&path) {
        Ok(repo) => {
            println!("found local repo");
            let head_reference = repo.head()?;
            let commit = head_reference.peel_to_commit()?;
            let _ = repo.branch(&version, &commit, true)?;
            let _ = repo.remote_set_url(&version, "")?;
            let remotes = repo.remotes()?;

            println!("djedou 3 remote: {:#?}", remotes.get(0));
            Ok(())
        },
        Err(_) => {
            panic!("This folder is not a git repository!");
        }
    }
}

/// Unlike regular "git init", this example shows how to create an initial empty
/// commit in the repository. This is the helper function that does that.
fn create_initial_commit(repo: &Repository, sig: &Signature) -> std::result::Result<Oid, E> {
    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
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
*/