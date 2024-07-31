mod generate_template;
mod dev;
mod build;
mod deploy;
mod serve;
mod args;
mod toml_config;
mod publish;

use clap::Parser;


pub(crate) use self::{
    args::*,
    generate_template::*,
    dev::*,
    build::*,
    serve::*,
    deploy::*,
    toml_config::*,
    publish::*
};
pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.subcommand {
        ArgSub::Init {name, mode} => {
            generate_template(&mode, &name)
        },
        ArgSub::Add {name: _} => {

        },
        ArgSub::Remove {name: _} => {
            
        },
        ArgSub::Serve {host: _, port: _} => {
            //serve(&host, port)
        },
        ArgSub::Deploy {host: _, port: _} => {
            //deploy(&host, port)
        },
        ArgSub::Dev => {
            //dev()
        },
        ArgSub::Build => {
            //build()
        },
        ArgSub::Publish => {
            let res = publish().await;
            println!("publis result: {res:#?}");
        }
    };

    Ok(())
}
