use std::io::Write;
use std::path::PathBuf;

use crate::engine::interface::PatinaInterface;
use crate::engine::{apply_patina_from_file, render_patina_from_file};
use clap::{Args, Parser, Subcommand};

/// The patina CLI renders files from templates and sets of variables as defined in patina toml files.
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
}

impl PatinaCli {
    /// Parse and return command line arguments
    pub fn parse_args() -> PatinaCli {
        PatinaCli::parse()
    }

    /// Run the CLI
    pub fn run(&self) {
        env_logger::Builder::new()
            .filter_level(self.global_options.verbosity.into())
            .init();

        match &self.command {
            Command::Render { options } => self.render(options),
            Command::Apply { options } => self.apply(options),
        }
    }

    fn render(&self, options: &PatinaCommandOptions) {
        self.handle_options(options);
        if let Err(e) = render_patina_from_file(&options.patina_path, self) {
            panic!("{:?}", e);
        };
    }

    fn apply(&self, options: &PatinaCommandOptions) {
        self.handle_options(options);
        if let Err(e) = apply_patina_from_file(&options.patina_path, self) {
            panic!("{:?}", e);
        }
    }

    fn handle_options(&self, options: &PatinaCommandOptions) {
        colored::control::set_override(!options.no_color);
    }
}

impl PatinaInterface for PatinaCli {
    fn output<S>(&self, s: S)
    where
        S: Into<String>,
    {
        print!("{}", s.into());
        let _ = std::io::stdout().flush();
    }
}
