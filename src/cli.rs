use crate::patina::Patina;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "patina", version)]
pub struct PatinaCli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(about = "Render a patina to stdout")]
    Render {
        #[clap(flatten)]
        options: PatinaCommandOptions,
    },

    #[clap(about = "Render and apply a patina")]
    Apply {
        #[clap(flatten)]
        options: PatinaCommandOptions,
    },
}

#[derive(Debug, Args)]
struct PatinaCommandOptions {
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
        let patina = Patina::from_toml_file(&options.patina_path);
        println!("{patina:#?}")
    }

    fn apply(_options: PatinaCommandOptions) {
        panic!("Not Implemented")
    }
}
