use std::env;
use std::process::exit;

mod changelog;
mod date;
mod git;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
    show_help();
    exit(0)
  }

  if args.contains(&"-V".to_owned()) || args.contains(&"--version".to_owned()) {
    show_version();
    exit(0)
  }

  changelog::generate(utils::parse_args(&args[1..]));
}

fn show_help() {
  print!(
    "
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
"
  )
}

fn show_version() {
  println!("chlog 0.3.0");
}
