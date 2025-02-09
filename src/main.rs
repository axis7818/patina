mod cli;
mod patina;

use cli::PatinaCli;

fn main() {
    let patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}
