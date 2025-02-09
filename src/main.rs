mod cli;
mod engine;
mod patina;
mod templating;
mod utils;

use cli::PatinaCli;

fn main() {
    let patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}
