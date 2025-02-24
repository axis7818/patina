# dotpatina

[![Crates.io](https://img.shields.io/crates/v/dotpatina)](https://crates.io/crates/dotpatina)
[![Docs Badge](https://github.com/axis7818/dotpatina/actions/workflows/generate-docs.yaml/badge.svg)](https://camerontaylor.dev/dotpatina/dotpatina/index.html)
[![CI Badge](https://github.com/axis7818/dotpatina/actions/workflows/continuous-integration.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml)
[![CD Badge](https://github.com/axis7818/dotpatina/actions/workflows/continuous-deployment.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-deployment.yaml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

dotpatina is a rust application for managing system dotfiles and configuration.

## Installation

### crates.io

`dotpatina` can be installed from its crate at crates.io.

```sh
cargo install dotpatina
```

## Usage

### Render

```sh
dotpatina render --help
```

```sh
Render a patina to stdout

Usage: patina render [OPTIONS] <PATINA_PATH>

Arguments:
  <PATINA_PATH>  The file path to the patina toml file

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
      --no-color    Disable colors
  -h, --help        Print help
```

### Apply

```sh
dotpatina apply --help
```

```sh
Render and apply a patina

Usage: patina apply [OPTIONS] <PATINA_PATH>

Arguments:
  <PATINA_PATH>  The file path to the patina toml file

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
      --no-color    Disable colors
      --no-input    Don't ask for user input
  -h, --help        Print help
```

## Examples

```sh
# Help
dotpatina --help
dotpatina render --help
dotpatina apply --help

# Render
dotpatina render examples/simple/patina.toml -vvv
dotpatina render examples/gitconfig/patina.toml -vvv

# Apply
dotpatina apply examples/simple/patina.toml -vvv
dotpatina apply examples/gitconfig/patina.toml -vvv
```
