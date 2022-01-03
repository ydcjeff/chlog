// Changelog generation and writing.

use std::fs;
use std::io;
use std::process;

use crate::git;
use crate::utils;

pub fn generate(args: (&str, &str, &str, &str)) {
  let (prepend, output, count, commit_path) = args;

  let url = git::get_remote_url();
  let mut releases = String::new();
  let tags_dates = git::get_tags_dates(commit_path, count);
  let len = tags_dates.len();

  for (i, tag_date) in tags_dates.iter().enumerate() {
    if i + 1 == len {
      break;
    }
    let (date, current_tag) = tag_date.split_at(10); // split YYYY-MM-DD date format
    let prev_tag = tags_dates[i + 1].split_at(10).1;
    let from_to = format!("{from}...{to}", from = prev_tag, to = current_tag);

    releases.push_str(&format!(
      "## [{tag}]({url}/compare/{from_to})\n\n_{date}_\n\n",
      tag = current_tag,
      url = url,
      from_to = from_to,
      date = date,
    ));

    let mut commits = git::get_commits(&from_to, commit_path);
    commits.sort_unstable();

    releases.push_str(&stringify_commits(commits, &url));

    // new line between releases if there isn't
    if !releases.ends_with("\n\n") {
      releases.push_str("\n\n");
    }
  }

  match write(prepend, output, &releases) {
    Ok(_) => (),
    Err(e) => {
      eprintln!("{}", e);
      process::exit(1)
    }
  }
}

fn stringify_commits(commits: Vec<String>, url: &str) -> String {
  let mut breaking = String::new();
  let mut fix = String::new();
  let mut deps = String::new();
  let mut deprecate = String::new();
  let mut dx = String::new();
  let mut docs = String::new();
  let mut feat = String::new();
  let mut perf = String::new();
  let mut refactor = String::new();
  let mut commits_list = String::new();

  for commit in commits {
    let (commit_hash, commit_type, commit_scope, commit_title, commit_body) =
      utils::process_commit(&commit);

    let mut template = format!(
      "- [`{hash}`]({url}/commit/{hash}) ",
      hash = commit_hash,
      url = url,
    );

    if !commit_scope.is_empty() {
      template.push_str(&format!("**{scope}** ", scope = commit_scope))
    }

    template.push_str(commit_title);

    if !commit_body.is_empty() {
      let body = utils::indent(commit_body);
      if !body.is_empty() {
        template.push_str(&format!("\n\n{body}", body = body))
      }
    }

    // new line between commit list
    template.push_str("\n\n");

    match commit_type {
      "breaking" => breaking.push_str(&template),
      "fix" => fix.push_str(&template),
      "deps" => deps.push_str(&template),
      "deprecate" => deprecate.push_str(&template),
      "dx" => dx.push_str(&template),
      "docs" => docs.push_str(&template),
      "feat" => feat.push_str(&template),
      "perf" => perf.push_str(&template),
      "refactor" => refactor.push_str(&template),
      _ => (),
    }
  }

  let mut add_if = |left: &str, right: String| {
    if !right.is_empty() {
      commits_list.push_str(&(left.to_owned() + &right))
    }
  };

  add_if("### BREAKING CHANGES\n\n", breaking);
  add_if("### Bug Fixes\n\n", fix);
  add_if("### Dependency Updates\n\n", deps);
  add_if("### Deprecations\n\n", deprecate);
  add_if("### Developer Experience\n\n", dx);
  add_if("### Documentation\n\n", docs);
  add_if("### Features\n\n", feat);
  add_if("### Performance Improvements\n\n", perf);
  add_if("### Refactoring\n\n", refactor);

  commits_list
}

fn write(prepend: &str, output: &str, to_write: &str) -> io::Result<()> {
  let placeholder = "<!-- CHLOG_SPLIT_MARKER -->\n";
  let path;
  let contents: String;

  if !output.is_empty() {
    path = output;
    contents = placeholder.to_owned() + to_write;
    println!("Generating changelog to {:?}...", path);
  } else if !prepend.is_empty() {
    path = prepend;
    let content = fs::read_to_string(prepend)?;
    let content: Vec<&str> = content.splitn(2, placeholder).collect();
    contents = content[0].to_owned() + placeholder + to_write + content[1];
    println!("Generating changelog and prepending to {:?}...", path);
  } else {
    println!("{}", to_write);
    return Ok(());
  }

  fs::write(path, contents)?;
  println!("Done!");
  Ok(())
}
