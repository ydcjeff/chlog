# chlog

<p>
  <a href="https://github.com/ydcjeff/chlog/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/ydcjeff/chlog/actions/workflows/ci.yml/badge.svg" alt="CI status">
  </a>
  <a href="https://crates.io/crates/chlog" target="_blank">
    <img src="https://badgen.net/crates/v/chlog" alt="crate version">
  </a>
  <a href="https://crates.io/crates/chlog" target="_blank">
    <img src="https://badgen.net/crates/d/chlog" alt="crate total download">
  </a>
</p>

> Universal changelog generator using conventional commit+ with monorepo
> support.

[![demo](https://raw.githubusercontent.com/ydcjeff/chlog/main/.github/screenshot.png)](https://github.com/ydcjeff/chlog/blob/main/.github/screenshot.png?raw=true)

**chlog** can generate the changelog from the conventional commits with a few
extra commit types. The supported commit types are:

- `fix` -> Bug Fixes
- `deps` -> Dependency Updates
- `deprecate` -> Deprecations (Deprecations are important for users as putting
  under refactor is hard to find)
- `dx` -> Developer Experience
- `docs` -> Documentation
- `feat` -> Features
- `perf` -> Performance Improvements
- `refactor` -> Refactoring

If there is `!` after the commit types or commit scope, those commits will be
under BREAKING CHANGES section.

## Installation

Binary releases can be downloaded at
[GitHub release page](https://github.com/ydcjeff/chlog/releases/latest) or if
you want to install with `cargo`:

```sh
cargo install chlog
```

## Usage

**chlog** has command line options for generating and prepending changelogs.

For the first release, you can run with:

```sh
chlog -o CHANGELOG.md -r 0 -t v0.1.0
```

For the subsequent release,

```sh
chlog -o CHANGELOG.md -t v0.2.0
```

For the packages with monorepo, you can use `commit-path` option. It will
generate the changelog scoped to that package.

```sh
chlog -o CHANGELOG.md -t v0.3.0 -r 2 --commit-path crates/scope-crate
```

CLI:

```console
  chlog

  Description:
    Universal changelog generator using conventional commit+
    with monorepo support

  Usage:
    $ chlog [options]

  Example:
    $ chlog -o CHANGELOG.md -t v1.0.0
    $ chlog -o CHANGELOG.md -t v1.0.0
    $ chlog -o CHANGELOG.md -t v1.0.0 -r 2
    $ chlog -o CHANGELOG.md -t v1.0.0 -r 2 --commit-path crates/scope-crate

  Options:
    -t  <string>          Tag name for the next release
    -r  <number>          Number of releases to generate the changelog
                          If 0, the whole changelog will be generated
                          (i.e. first release) (default: 1)
    -o  <file>            File to write the generated changelog
                          It will prepend the changelogs if the file exists
                          otherwise, will create a new one
    --commit-path <path>  Generate a changelog scoped to a specific directory

  Flags:
    -h, --help            Show this message
    -V, --version         Show version number

  Source: https://github.com/ydcjeff/chlog
```

## Contribution

- Make sure you have installed [Rust](https://www.rust-lang.org/tools/install).
- Setup git hook

  ```sh
  git config core.hookspath .githooks
  ```

## LICENSE

[MIT](./LICENSE)
