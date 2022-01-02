use crate::git;

pub fn indent(s: &str) -> String {
  let mut result = String::new();

  for line in s.lines() {
    if line.is_empty() {
      result.push('\n')
    } else if !line.to_lowercase().starts_with("co-author") {
      result.push_str(&format!("  {}\n", line))
    }
  }
  result.trim_end().to_owned()
}

pub fn process_commit(commit: &str) -> (&str, &str, &str, &str, &str) {
  let commit: Vec<&str> = commit.splitn(3, git::CHLOG_MID).collect();
  let commit_subject = commit[0];
  let commit_body = commit[1];
  let commit_hash = commit[2];

  let commit_subject: Vec<&str> = commit_subject.splitn(2, ": ").collect();
  let commit_prefix = commit_subject[0]; // e.g. fix(scope) fix(scope)! fix fix!
  let commit_title = commit_subject[1];

  let mut commit_scope = "";
  let mut commit_type = commit_prefix;

  if let (Some(lidx), Some(ridx)) =
    (commit_prefix.find('('), commit_prefix.find(')'))
  {
    commit_scope = &commit_prefix[lidx..ridx + 1];
    commit_type = &commit_prefix[..lidx]
  }

  if commit_prefix.ends_with('!') {
    commit_type = "breaking"
  }

  (
    commit_hash,
    commit_type,
    commit_scope,
    commit_title,
    commit_body,
  )
}

pub fn parse_args(args: &[String]) -> (&str, &str, &str, &str) {
  let mut prepend = "";
  let mut output = "";
  let mut count = "1";
  let mut commit_path = ".";

  let mut args = args.iter();

  loop {
    let next = args.next();
    match next {
      Some(v) => match v.as_str() {
        "-p" => {
          prepend = args.next().unwrap();
        }
        "-o" => {
          output = args.next().unwrap();
        }
        "-c" => {
          count = args.next().unwrap();
        }
        "--commit-path" => {
          commit_path = args.next().unwrap();
        }
        _ => (),
      },
      None => break,
    }
  }

  (prepend, output, count, commit_path)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_indent() {
    let string = "
This is a body text.

List items

- one
- two
- three
";

    let expected = "
  This is a body text.

  List items

  - one
  - two
  - three";

    assert_eq!(indent(string), expected);

    assert_eq!(
      indent("First line.\nSecond line."),
      "  First line.\n  Second line."
    );

    assert_eq!(
      indent("First line.\n\n\nSecond line.\n"),
      "  First line.\n\n\n  Second line."
    );
  }

  #[test]
  fn test_process_commit() {
    // normal
    let string = format!("feat: test{p}{p}123456", p = git::CHLOG_MID);
    assert_eq!(process_commit(&string), ("123456", "feat", "", "test", ""));

    // normal with body text
    let string = format!("feat: test{p}body text{p}123456", p = git::CHLOG_MID);
    assert_eq!(
      process_commit(&string),
      ("123456", "feat", "", "test", "body text")
    );

    // normal scope
    let string = format!("docs(config): test{p}{p}123456", p = git::CHLOG_MID);
    assert_eq!(
      process_commit(&string),
      ("123456", "docs", "(config)", "test", "")
    );

    // normal scope with body text
    let string = format!(
      "docs(config): test{p}body text{p}123456",
      p = git::CHLOG_MID
    );
    assert_eq!(
      process_commit(&string),
      ("123456", "docs", "(config)", "test", "body text")
    );

    // breaking change
    let string =
      format!("fix!: test{p}\nbody text{p}123456", p = git::CHLOG_MID);
    assert_eq!(
      process_commit(&string),
      ("123456", "breaking", "", "test", "\nbody text")
    );

    // breaking change with scope
    let string = format!(
      "fix(scope)!: test{p}\nbody text{p}123456",
      p = git::CHLOG_MID
    );
    assert_eq!(
      process_commit(&string),
      ("123456", "breaking", "(scope)", "test", "\nbody text")
    );
  }

  #[test]
  fn test_parse_args() {
    let args = ["-p", "prepend.md", "-o", "output.md"].map(String::from);
    assert_eq!(parse_args(&args), ("prepend.md", "output.md", "1", "."));

    let args =
      ["-p", "prepend.md", "-o", "output.md", "-c", "2"].map(String::from);
    assert_eq!(parse_args(&args), ("prepend.md", "output.md", "2", "."));

    let args = [
      "-p",
      "prepend.md",
      "-o",
      "output.md",
      "-c",
      "2",
      "--commit-path",
      "test",
    ]
    .map(String::from);
    assert_eq!(parse_args(&args), ("prepend.md", "output.md", "2", "test"));
  }
}
