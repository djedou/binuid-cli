use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Subcommand)]
pub(crate) enum ArgSub {
    /// Initiate your project, ex: binuid init --mode=mode --name=name
    #[command(arg_required_else_help = true)]
    Init {
        /// Project mode
        #[arg(
            short,
            long,
            require_equals = true,
            value_name = "MODE",
            num_args = 0..=1,
            default_value_t = Mode::Bin,
            default_missing_value = "bin",
            value_enum
        )]
        mode: Mode,
        /// Project name
        #[arg(required = true, short, long)]
        name: String
    },
    /// Add new library to your project
    #[command(arg_required_else_help = true)]
    Add {
        /// Library name to add
        #[arg(required = true, short, long)]
        name: String
    },
    /// Remove a library from your project
    #[command(arg_required_else_help = true)]
    Remove {
        /// Library name to remove
        #[arg(required = true, short, long)]
        name: String
    },
    /// Serve your project for develop
    Serve {
        #[arg(short, long)]
        #[arg(long, default_value="0.0.0.0")]
        host: String,
        #[arg(short, long)]
        #[arg(long, default_value="3000")]
        port: u16
    },
    /// Run production server
    Deploy {
        #[arg(short, long)]
        #[arg(long, default_value="0.0.0.0")]
        host: String,
        #[arg(short, long)]
        #[arg(long, default_value="3000")]
        port: u16
    },
    /// Run for develop
    Dev,
    /// Build for production
    Build,
    /// Publish your project for others to use it
    Publish
}


#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Bin,
    Lib,
    Ws,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Parser)]
#[command(name = "binuid")]
#[command(about = "binuid command line interface", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) subcommand: ArgSub,
}