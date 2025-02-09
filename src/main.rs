mod cli;
mod patina;
mod patina_yaml;

use cli::PatinaCli;

fn main() {
    let patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}
