use clap::{Parser, Subcommand};

mod aws_sdk;
mod build;
mod check;
mod clean;
mod clippy;
mod doc;
mod fmt;
mod init;
mod run;
mod test;

use std::path::PathBuf;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "acp")]
#[command(about = "AppCove Platform", long_about = None)]

struct Cli {
    #[clap(
        short = 'C',
        long,
        help = "Run in this directory instead of the current directory"
    )]
    current_directory: Option<PathBuf>,

    #[clap(
        long = "require-version",
        help = "require this version to be the one running or die with an error"
    )]
    require_version: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Init
    #[clap(about = "Initialize the workspace")]
    Init,

    #[clap(about = "cargo build -p ... (after code generation)")]
    Build,

    #[clap(about = "cargo run -p ... (after code generation)")]
    Run,

    // Clean
    #[clap(about = "cargo clean -p ...")]
    Clean,

    // Clippy
    #[clap(about = "cargo clippy -p ...")]
    Clippy,

    // Check
    #[clap(about = "cargo check -p ...")]
    Check {
        #[clap(long, help = "Check all crates in the workspace")]
        all: bool,
    },

    // Doc
    #[clap(about = "cargo doc -p ...")]
    Doc,

    // Fmt
    #[clap(about = "cargo fmt -p ...")]
    Fmt,

    // Test
    #[clap(about = "cargo test -p ...")]
    Test,

    #[clap(about = "View info on this workspace")]
    Workspace,

    #[clap(about = "Do nothing but parse the input files")]
    Nothing,

    #[clap(about = "Manage the aws-sdk custom builds")]
    AwsSdk {
        #[clap(subcommand)]
        subcommand: AwsSdkCommands,
    },
}

#[derive(Debug, Parser)]
struct AllTheArgs {
    args: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum AwsSdkCommands {
    #[clap(about = "Clone smithy-rs and set on proper commit")]
    Clone,

    #[clap(
        about = "Set smithy-rs repo to proper state, modify input files, and build the aws-sdk"
    )]
    Build,

    #[clap(about = "Clean-up temp files")]
    Clean,
}

pub async fn main() {
    let args = Cli::parse();

    // perform version check
    if let Some(require_version) = args.require_version {
        let current_version = env!("CARGO_PKG_VERSION");
        if current_version != require_version {
            eprintln!(
                "Error: `acp` version mismatch. Require version `{}`, but this is version `{}`.\n\nRun `./acp init` to update.",
                require_version, current_version
            );
            std::process::exit(1);
        }
    }

    // get the current working directory
    let cwd = match args.current_directory {
        Some(ref cwd) => cwd.to_owned(),
        None => std::env::current_dir().expect("could not get current directory"),
    };

    let workspace: &'static crate::Workspace = Box::leak(Box::new(crate::Workspace::init(&cwd)));

    match args.command {
        Commands::Init => {
            self::init::init(workspace);
        }

        Commands::Build => {
            self::build::build(workspace);
        }

        Commands::Run => {
            self::run::run(workspace);
        }

        Commands::Clean => {
            self::clean::clean(workspace);
        }

        Commands::Clippy => {
            self::clippy::clippy(workspace);
        }

        Commands::Check { all } => {
            self::check::check(workspace, all);
        }

        Commands::Doc => {
            self::doc::doc(workspace);
        }

        Commands::Fmt => {
            self::fmt::fmt(workspace);
        }

        Commands::Test => {
            self::test::test(workspace);
        }

        Commands::Workspace => {
            println!("Workspace: {:#?}", workspace);
        }

        Commands::Nothing => {
            println!("Input files parsed");
        }

        Commands::AwsSdk { subcommand } => match subcommand {
            AwsSdkCommands::Clone => {
                self::aws_sdk::clone(workspace);
            }
            AwsSdkCommands::Build => {
                self::aws_sdk::build(workspace);
            }
            AwsSdkCommands::Clean => {
                self::aws_sdk::clean(workspace);
            }
        },
    }
}
