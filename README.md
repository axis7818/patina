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

`dotpatina` takes templated configuration files (using handlebars templating), rendering configuration files, and applying them to target locations on the file system.

### Applying a Patina

`dotpatina` accepts a path to a Patina toml file that defines files and variables used for rendering. Separate variables toml files can be provided to overlay variable customizations.

![gif of applying a new patina](./examples/gitconfig/apply-new-patina.gif)

A diff view is presented with each `apply` command to show only lines that will change.

<details>
<summary>apply change</summary>

```sh
❱ dotpatina apply patina.toml --vars other-vars.toml

/Users/cameron/Repos/github.com/axis7818/dotpatina/output/.gitconfig
   1  1 | [user]
-  2    |     email = <user@email.com>
-  3    |     name = <User Name>
+     2 |     email = <different@email.com>
+     3 |     name = <Different User>
   4  4 | [alias]
   5  5 |     lg = !git lg1
   6  6 |     lg1 = !git lg1-specific --all
   7  7 |     lg2 = !git lg2-specific --all

... 11 unchanged lines



/Users/cameron/Repos/github.com/axis7818/dotpatina/output/lazygit.config
13 lines, no changes detected

Do you want to continue? (y/n): y

Applying patina files
   /Users/cameron/Repos/github.com/axis7818/dotpatina/output/.gitconfig ✓
   /Users/cameron/Repos/github.com/axis7818/dotpatina/output/lazygit.config ✓
Done
```

</details>

Files are only written when there are changes.

<details>
<summary>apply no changes</summary>

```sh
❱ dotpatina apply patina.toml --vars other-vars.toml

/Users/cameron/Repos/github.com/axis7818/dotpatina/output/.gitconfig
18 lines, no changes detected


/Users/cameron/Repos/github.com/axis7818/dotpatina/output/lazygit.config
13 lines, no changes detected

No file changes detected in the patina%
```

</details>

### Render a Patina

`dotpatina` also supports rendering Patina files without writing to the target locations.

```sh
❱ cd examples/gitconfig

❱ dotpatina render patina.toml --vars vars.toml
Rendered 2 files


gitconfig.hbs
[user]
    email = <user@email.com>
    name = <User Name>
[alias]
    lg = !git lg1
    lg1 = !git lg1-specific --all
    lg2 = !git lg2-specific --all
    lg3 = !git lg3-specific --all
[pager]
    branch = false
[core]
        editor = vim
[pull]
        rebase = false
[init]
        defaultBranch = main
[fetch]
        prune = true


lazygit.config.yml
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
