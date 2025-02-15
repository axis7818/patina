# Patina

[![CI Badge](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml/badge.svg?branch=main)](https://github.com/axis7818/patina/actions/workflows/continuous-integration.yaml)
[![Docs Badge](https://github.com/axis7818/patina/actions/workflows/generate-docs.yaml/badge.svg)](https://camerontaylor.dev/patina/patina/index.html)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Patina is a rust application for managing system dotfiles and configuration.

## Examples

```sh
cargo run -- --help
cargo run -- render --help
cargo run -- apply --help

# Render
cargo run -- render examples/simple/patina.toml -vvv
cargo run -- render examples/gitconfig/patina.toml -vvv

# Apply
cargo run -- apply examples/simple/patina.toml -vvv
cargo run -- apply examples/gitconfig/patina.toml -vvv
```
