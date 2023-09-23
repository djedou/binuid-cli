mod generate_template;
mod dev;
mod build;
mod deploy;
mod serve;
mod args;

use clap::Parser;


pub(crate) use self::{
    args::*,
    generate_template::*,
    dev::*,
    build::*,
    serve::*,
    deploy::*
};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.subcommand {
        ArgSub::Init(InitArgs{name}) => {
            generate_template(&name);
        },
        ArgSub::Serve(ServerArgs{host, port}) => {
            serve(&host, port)
        },
        ArgSub::Deploy(ServerArgs{host, port}) => {
            deploy(&host, port)
        },
        ArgSub::Dev => {
            dev()
        },
        ArgSub::Build => {
            build()
        }
    };

    Ok(())
}
