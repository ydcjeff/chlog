// Get information from git.

use std::process::{exit, Command, Stdio};
use std::str;

const CHLOG_END: &str = "_____CHLOG_END____";
pub const CHLOG_MID: &str = "_____CHLOG_MID_____";

pub fn git(args: &[&str]) -> String {
  let out = Command::new("git")
    .args(args)
    .stderr(Stdio::inherit())
    .output()
    .unwrap_or_else(|_| panic!("failed to execute git {}.", args.join(" ")));

  if !out.status.success() {
    exit(1)
  }

  str::from_utf8(&out.stdout).unwrap().trim().to_owned()
}

pub fn git_log(args: &[&str]) -> Vec<String> {
  let mut log_args = vec!["log"];
  log_args.extend_from_slice(args);
  let out = git(&log_args);

  out
    .split(CHLOG_END)
    .filter(|i| !i.is_empty())
    .map(|i| i.trim())
    .map(String::from)
    .collect()
}

pub fn get_commits(from_to: &str, commit_path: &str) -> Vec<String> {
  git_log(&[
    from_to,
    // Hopefully this is safe to exclude merge commits, usually they are
    // "Merge pull request..." or "Merge main branch..."
    "--no-merges",
    &format!("--format=%s{}%b{}%h{}", CHLOG_MID, CHLOG_MID, CHLOG_END),
    "--grep=^(fix|deps|deprecate|dx|docs|feat|perf|refactor)(.+)?!?: ",
    "-E",
    "--",
    commit_path,
  ])
}

pub fn get_tags_dates(commit_path: &str, count: usize) -> Vec<String> {
  let format = format!("--format=%ad%S{}", CHLOG_END);
  let mut args = vec![
    "--date=short",
    "--tags",
    &format,
    "--no-walk",
    "--",
    commit_path,
  ];

  let number: String;
  if count != 0 {
    // -n needs to appear before --no-walk
    number = (count + 1).to_string();
    args.splice(0..0, ["-n", &number]);
  }

  git_log(&args)
}

pub fn get_remote_url() -> String {
  let mut url = git(&["remote", "get-url", "origin"]);

  // ssh url
  if url.starts_with("git@") {
    url = url.replacen(":", "/", 1).replacen("git@", "https://", 1)
  }

  if let Some(i) = url.strip_suffix(".git") {
    i.to_owned()
  } else {
    url
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_remote_url() {
    let url = get_remote_url();

    assert!(url.starts_with("https://github.com"));
    assert!(url.ends_with("chlog"));
  }
}
