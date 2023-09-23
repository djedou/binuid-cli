use clap::{
    error::{Error, ErrorKind}, 
    ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand
};

#[derive(Parser, Debug)]
pub(crate) struct InitArgs {
    /// Your project name
    #[arg(short, long)]
    pub(crate) name: String,
}

#[derive(Parser, Debug)]
pub(crate) struct ServerArgs {
    #[arg(long, default_value="0.0.0.0")]
    pub(crate) host: String,
    #[arg(long, default_value="3000")]
    pub(crate) port: u16
}

#[derive(Debug)]
pub(crate) enum ArgSub {
    /// Initiate your project name
    Init(InitArgs),
    /// Serve your project for develop
    Serve(ServerArgs),
    /// Run production server
    Deploy(ServerArgs),
    /// Build for develop
    Dev,
    /// Build for production
    Build
}

impl FromArgMatches for ArgSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("init", args)) => Ok(Self::Init(InitArgs::from_arg_matches(args)?)),
            Some(("serve", args)) => Ok(Self::Serve(ServerArgs::from_arg_matches(args)?)),
            Some(("deploy", args)) => Ok(Self::Deploy(ServerArgs::from_arg_matches(args)?)),
            Some(("dev", _)) => Ok(Self::Dev),
            Some(("build", _)) => Ok(Self::Build),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `init` and `serve` `deploy` `dev` `build`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `init` and `serve` `deploy` `dev` `build`",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("init", args)) => *self = Self::Init(InitArgs::from_arg_matches(args)?),
            Some(("serve", args)) => *self = Self::Serve(ServerArgs::from_arg_matches(args)?),
            Some(("deploy", args)) => *self = Self::Deploy(ServerArgs::from_arg_matches(args)?),
            Some(("dev", _)) => *self = Self::Dev,
            Some(("build", _)) => *self = Self::Build,
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `init` and `serve`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for ArgSub {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(ServerArgs::augment_args(Command::new("serve")))
            .subcommand(ServerArgs::augment_args(Command::new("deploy")))
            .subcommand(Command::new("dev"))
            .subcommand(Command::new("build"))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(ServerArgs::augment_args(Command::new("serve")))
            .subcommand(ServerArgs::augment_args(Command::new("deploy")))
            .subcommand(Command::new("dev"))
            .subcommand(Command::new("build"))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "init" | "serve" | "deploy" | "dev" | "build")
    }
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) subcommand: ArgSub,
}
