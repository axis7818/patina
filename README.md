# dotpatina

[![Crates.io](https://img.shields.io/crates/v/dotpatina)](https://crates.io/crates/dotpatina)
[![Docs Badge](https://github.com/axis7818/dotpatina/actions/workflows/generate-docs.yaml/badge.svg)](https://camerontaylor.dev/dotpatina/dotpatina/index.html)
[![CI Badge](https://github.com/axis7818/dotpatina/actions/workflows/continuous-integration.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml)
[![CD Badge](https://github.com/axis7818/dotpatina/actions/workflows/continuous-deployment.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-deployment.yaml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

dotpatina is a rust application for managing system dotfiles and configuration.

## Installation

`dotpatina` can be installed from its crate at crates.io.

```sh
cargo install dotpatina
```

### Verify Installation

```sh
# View dotpatina version
❱ dotpatina --version
```

```sh
# View dotpatina usage info
❱ dotpatina --help
The patina CLI renders files from templates and sets of variables as defined in patina toml files

Usage: dotpatina [OPTIONS] <COMMAND>

Commands:
  render  Render a patina to stdout
  apply   Render and apply a patina
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```

## Usage

`dotpatina` takes templated configuration files (using handlebars templating), rendering configuration files, and applying them to target locations on the file system.

### Patina File

```toml
# Metadata fields describe the Patina
name = "git-patina"
description = "A Patina for git tooling"

# Variables are free-form and can be defined for the whole Patina.
# Or, variables can be loaded from separate files from the command line.
[vars]
editor = "vim"

# A list of files define a template and target file.
# The template is a handlebar template (or plain file) that is processed.
# The target is the system location to store the rendered template.
[[files]]
template = "gitconfig.hbs"
target = "../../output/.gitconfig"

[[files]]
template = "lazygit.config.yml"
target = "../../output/lazygit.config"
```

### Variables Files

Variables can be stored in separate toml files. Variables are free-form and overlay on top of the base Patina variables.

This is useful when variables need to change based on the machine Patinas are being applied to.

```toml
[user]
name = "User Name"
email = "user@email.com"
```

### Template Files

Patina templates are defined using handlebars templates. Or, they can be raw files if no templating is required.

#### Handlebar Template (gitconfig.hbs)

```hbs
[user]
    email = <{{ user.email }}>
    name = <{{ user.name }}>
[pager]
    branch = false
[core]
	editor = {{ editor }}
[pull]
	rebase = false
[init]
	defaultBranch = main
[fetch]
	prune = true
```

#### Raw File (lazygit.config.yml)

```yml
gui:
  showBottomLine: false
  showCommandLog: false
  showPanelJumps: false
  border: rounded
  showNumstatInFilesView: true

customCommands:
  - key: "U"
    command: "git submodule update --init --recursive"
    context: "files, localBranches, status"
    description: "Update submodules"
    stream: true
```

### Render a Patina

`dotpatina` supports rendering Patina files to stdout for previewing.

Provide a path to a Patina toml file that defines files and variables used for rendering. Separate variables toml files can be provided to overlay variable customizations.

![gif of rendering a patina](./examples/gitconfig/render-patina.gif)

### Applying a Patina

Applying a Patina is how rendered files get written to the file system.

![gif of applying a new patina](./examples/gitconfig/apply-new-patina.gif)

A diff view is presented with each `apply` command to show only lines that will change. This could be due to changing the template, or using a different set of variables.

![gif of applying a patina with other variables](./examples/gitconfig/apply-other-vars-patina.gif)
