use std::io::Write;
use std::path::PathBuf;

use crate::engine::interface::PatinaInterface;
use crate::engine::{apply_patina_from_file, render_patina_from_file};
use clap::{Args, Parser, Subcommand};

/// The patina CLI renders files from templates and sets of variables as defined in patina toml files.
#[derive(Parser, Debug)]
#[clap(name = "dotpatina", version)]
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
    /// The verbosity level of the CLI
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
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

        /// Don't ask for user input
        #[clap(long = "no-input")]
        no_input: bool,
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

    /// Disable colors
    #[clap(long = "no-color")]
    no_color: bool,

    /// The list of tags to filter on
    #[clap(short = 't', long = "A list of tags to filter on")]
    tags: Vec<String>,
}

impl PatinaCli {
    /// Parse and return command line arguments
    pub fn parse_args() -> PatinaCli {
        PatinaCli::parse()
    }

    /// Run the CLI
    pub fn run(&mut self) {
        env_logger::Builder::new()
            .filter_level(self.global_options.verbosity.into())
            .init();

        let pi = CliPatinaInterface::new();
        let result = match &self.command {
            Command::Render { options } => render_patina_from_file(&options.patina_path, &pi),
            Command::Apply { options, no_input } => {
                apply_patina_from_file(&options.patina_path, &pi, *no_input)
            }
        };

        if let Err(e) = result {
            panic!("{:?}", e)
        }
    }
}

struct CliPatinaInterface {}

impl CliPatinaInterface {
    fn new() -> CliPatinaInterface {
        CliPatinaInterface {}
    }
}

impl PatinaInterface for CliPatinaInterface {
    fn output<S>(&self, s: S)
    where
        S: Into<String>,
    {
        print!("{}", s.into());
        let _ = std::io::stdout().flush();
    }
}
