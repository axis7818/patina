# Patina

[![CI Badge](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml)
[![Docs Badge](https://github.com/axis7818/patina/actions/workflows/generate-docs.yaml/badge.svg)](https://camerontaylor.dev/patina/patina/index.html)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Patina is a rust application for managing system dotfiles and configuration.

## Usage

### Render

```sh
❱ patina render --help
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
❱ patina apply --help
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
patina --help
patina render --help
patina apply --help

# Render
patina render examples/simple/patina.toml -vvv
patina render examples/gitconfig/patina.toml -vvv

# Apply
patina apply examples/simple/patina.toml -vvv
patina apply examples/gitconfig/patina.toml -vvv
```
