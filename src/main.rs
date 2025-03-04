//! The Patina CLI is a simple command line application written in rust for processing
//! Patina template files and applying them to locations on the file system.
//!
//! `dotpatina` takes templated configuration files (using handlebars templating), rendering configuration files, and applying them to target locations on the file system. This information is provided by a Patina toml file.
//!
//! ### Patina File
//!
//! This is an example Patina file for git tooling.
//!
//! ```toml
//! # Metadata fields describe the Patina
//! name = "git-patina"
//! description = "A Patina for git tooling"
//!
//! # Variables are free-form and can be defined for the whole Patina.
//! # Or, variables can be loaded from separate files from the command line.
//! [vars]
//! editor = "vim"
//!
//! # A list of files define a template and target file.
//! # The template is a handlebar template (or plain file) that is processed.
//! # The target is the system location to store the rendered template.
//! [[files]]
//! template = "gitconfig.hbs"
//! target = "../../output/.gitconfig"
//!
//! [[files]]
//! template = "lazygit.config.yml"
//! target = "../../output/lazygit.config"
//! ```
//!
//! ### Variables Files
//!
//! Variables can be stored in separate toml files. Variables are free-form and overlay on top of the base Patina variables.
//!
//! This is useful when variables need to change based on the machine Patinas are being applied to.
//!
//! ```toml
//! [user]
//! name = "User Name"
//! email = "user@email.com"
//! ```
//!
//! ### Template Files
//!
//! Patina templates are defined using handlebars templates. Or, they can be raw files if no templating is required.
//!
//! #### Handlebar Template
//!
//! Templates are rendered using the variables provided directly in the Patina file and passed as separate variables files. In this example, `editor` is provided in the Patina file but `user.email` and `user.name` are provided in a separate variables file.
//!
//! `gitconfig.hbs`
//!
//! ```hbs
//! [user]
//!     email = <{{ user.email }}>
//!     name = <{{ user.name }}>
//! [pager]
//!     branch = false
//! [core]
//!     editor = {{ editor }}
//! [pull]
//!     rebase = false
//! [init]
//!     defaultBranch = main
//! [fetch]
//!     prune = true
//! ```
//!
//! #### Raw File
//!
//! Raw files without templating work as well.
//!
//! `lazygit.config.yml`
//!
//! ```yml
//! gui:
//!   showBottomLine: false
//!   showCommandLog: false
//!   showPanelJumps: false
//!   border: rounded
//!   showNumstatInFilesView: true
//!
//! customCommands:
//!   - key: "U"
//!     command: "git submodule update --init --recursive"
//!     context: "files, localBranches, status"
//!     description: "Update submodules"
//!     stream: true
//! ```

use cli::PatinaCli;

mod cli;
mod diff;
mod engine;
mod patina;
mod templating;
mod utils;

/// Main entry point for the application.
/// This launches the CLI interface.
fn main() {
    let mut patina_cli = PatinaCli::parse_args();
    patina_cli.run();
}

#[cfg(test)]
mod tests {
    pub mod test_utils;
}
