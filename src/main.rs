//! The Patina CLI is a simple command line application written in rust for processing Patina template files and applying them to locations on the file system.

mod cli;
mod engine;
mod patina;
mod templating;
mod utils;

use cli::PatinaCli;

/// Main entry point for the application.
/// This launches the CLI interface.
fn main() {
    let patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}
