use crate::engine::render_patina_from_file;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Clap definition of the Patina CLI
#[derive(Parser, Debug)]
#[clap(name = "patina", version)]
pub struct PatinaCli {
    /// Global options apply to all subcommands
    #[clap(flatten)]
    global_options: GlobalOptions,

    /// The specified command to run
    #[clap(subcommand)]
    command: Command,
}

/// Options that apply globally to the CLI
#[derive(Debug, Args)]
struct GlobalOptions {
    /// When set, the CLI will print more information to the console
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

/// The available commands for the CLI
#[derive(Debug, Subcommand)]
enum Command {
    /// Render a patina to stdout
    #[clap(about = "Render a patina to stdout")]
    Render {
        /// Command line options
        #[clap(flatten)]
        options: PatinaCommandOptions,
    },

    /// Render and apply a patina
    #[clap(about = "Render and apply a patina")]
    Apply {
        /// Command line options
        #[clap(flatten)]
        options: PatinaCommandOptions,
    },
}

/// Options that apply to patina subcommands
#[derive(Debug, Args)]
struct PatinaCommandOptions {
    /// Included global options
    #[clap(flatten)]
    global_options: GlobalOptions,

    /// The file path to the patina toml file
    patina_path: PathBuf,
}

impl PatinaCli {
    /// Parse and return command line arguments
    pub fn parse_args() -> PatinaCli {
        PatinaCli::parse()
    }

    /// Run the CLI
    pub fn run(self) {
        match self.command {
            Command::Render { options } => PatinaCli::render(options),
            Command::Apply { options } => PatinaCli::apply(options),
        }
    }

    fn render(options: PatinaCommandOptions) {
        let patina_render = match render_patina_from_file(options.patina_path) {
            Ok(patina_render) => patina_render,
            Err(e) => panic!("{:?}", e),
        };

        println!("printing patina render");
        patina_render.iter().for_each(|p| {
            println!("{:#?}", p);
        });
    }

    fn apply(_options: PatinaCommandOptions) {
        panic!("Not Implemented")
    }
}
