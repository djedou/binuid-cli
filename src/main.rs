mod template;
mod dev;
mod build;
mod deploy;
mod serve;
mod args;
mod config;
mod publish;
mod servers;
mod compiler;

use binuid_shared::{
    clap::Parser,
    tokio
};
use binuid_shared_wasm::tracing::Level;
use binuid_shared::tracing_subscriber;

pub(crate) use self::{
    args::*,
    template::*,
    dev::*,
    build::*,
    serve::*,
    deploy::*,
    config::*,
    publish::*
};
pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let cli = Cli::parse();
    match cli.subcommand {
        ArgSub::Init {name, mode} => {
            generate_template(&mode, &name)
        },
        ArgSub::Add {name: _} => {

        },
        ArgSub::Remove {name: _} => {
            
        },
        ArgSub::Serve {host, port} => {
            serve(&host, port).await
        },
        ArgSub::Deploy {host: _, port: _} => {
            //deploy(&host, port)
        },
        ArgSub::Dev => {
            dev()
        },
        ArgSub::Build => {
            let _ = build()?;
        },
        ArgSub::Publish => {
            let res = publish().await;
            println!("publis result: {res:#?}");
        }
    };

    Ok(())
}
