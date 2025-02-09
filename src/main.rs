mod cli;
mod engine;
mod errors;
mod patina;
mod templating;

use cli::PatinaCli;

fn main() {
    let patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}
