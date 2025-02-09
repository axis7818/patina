use crate::engine::render_patina_from_file;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "patina", version)]
pub struct PatinaCli {
    #[clap(flatten)]
    global_options: GlobalOptions,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct GlobalOptions {
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
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
    #[clap(flatten)]
    global_options: GlobalOptions,

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
