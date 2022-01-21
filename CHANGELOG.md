# chlog changelog

<!-- CHLOG_SPLIT_MARKER -->

## [v0.3.0](https://github.com/ydcjeff/chlog/compare/v0.2.5...v0.3.0)

_2022-01-21_

### BREAKING CHANGES

- [`e3340d8`](https://github.com/ydcjeff/chlog/commit/e3340d8) changelog writing
  is now unified under single -o option (#7)

  `-p` is now removed. Instead use `-o` as usual. It will create
  the output file if the file does not exist and write the changelog
  to it. If the file exists, the generated changelog will be
  prepended to the file.

## [v0.2.5](https://github.com/ydcjeff/chlog/compare/v0.2.4...v0.2.5)

_2022-01-03_

## [v0.2.4](https://github.com/ydcjeff/chlog/compare/v0.2.3...v0.2.4)

_2022-01-03_

## [v0.2.3](https://github.com/ydcjeff/chlog/compare/v0.2.2...v0.2.3)

_2022-01-03_

## [v0.2.2](https://github.com/ydcjeff/chlog/compare/v0.2.1...v0.2.2)

_2022-01-03_

## [v0.2.1](https://github.com/ydcjeff/chlog/compare/v0.2.0...v0.2.1)

_2022-01-03_

## [v0.2.0](https://github.com/ydcjeff/chlog/compare/v0.1.0...v0.2.0)

_2022-01-03_

### Bug Fixes

- [`dbb1aa4`](https://github.com/ydcjeff/chlog/commit/dbb1aa4) do not
  canonicalize the output file

- [`a04f0de`](https://github.com/ydcjeff/chlog/commit/a04f0de) get today date
  from the system for release date

### Features

- [`189dbb3`](https://github.com/ydcjeff/chlog/commit/189dbb3) add -t option for
  next release tag name

- [`ac2a90e`](https://github.com/ydcjeff/chlog/commit/ac2a90e) exit on unknown
  options

- [`e8d8d29`](https://github.com/ydcjeff/chlog/commit/e8d8d29) generate
  changelog for unreleased

### Refactoring

- [`463625f`](https://github.com/ydcjeff/chlog/commit/463625f) change -c option
  to -r to align with name "releases"

## [v0.1.0](https://github.com/ydcjeff/chlog/compare/46a3b87...v0.1.0)

_2022-01-03_

### Features

- [`eebe1eb`](https://github.com/ydcjeff/chlog/commit/eebe1eb) add help &
  version flag

- [`f4eadbf`](https://github.com/ydcjeff/chlog/commit/f4eadbf) get required
  information from git

- [`0e4318e`](https://github.com/ydcjeff/chlog/commit/0e4318e) read + write
  changelog file

- [`4ac1249`](https://github.com/ydcjeff/chlog/commit/4ac1249) skip Co-authored
  by messages

- [`b6a0547`](https://github.com/ydcjeff/chlog/commit/b6a0547) utils for
  processing commits & args
