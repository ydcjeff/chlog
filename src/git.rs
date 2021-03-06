// Get information from git.

use std::process::{exit, Command, Stdio};
use std::str;

use crate::date;

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

pub fn get_tags_dates(commit_path: &str, count: &str) -> Vec<String> {
  let format = format!("--format=%ad%S{}", CHLOG_END);
  let mut tags_dates = vec![format!("{}HEAD", date::today())];
  let mut args = vec![
    "--date=short",
    "--tags",
    &format,
    "--no-walk",
    "--",
    commit_path,
  ];

  if count != "0" {
    // -n needs to appear before --no-walk
    args.splice(0..0, ["-n", count]);
  }

  let logs = git_log(&args);
  tags_dates.extend(logs);

  if count == "0" {
    let first_commit =
      git(&["rev-list", "HEAD", "--max-parents=0", "--abbrev-commit"]);
    tags_dates.push(format!("0000-00-00{}", first_commit));
  }

  tags_dates
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
