use crate::git;
use crate::utils;

pub fn generate(args: (&str, &str, &str, &str)) {
  let (prepend, output, count, commit_path) = args;

  let tags_dates = git::get_tags_dates(commit_path, count);
  let len = tags_dates.len();

  for (i, tag_date) in tags_dates.iter().enumerate() {
    let (date, current_tag) = tag_date.split_at(10); // split YYYY-MM-DD date format
    let mut from_to = format!("{}", current_tag); // all commits

    // commits between 2 tags
    if i + 1 < len {
      let prev_tag = tags_dates[i + 1].split_at(10).1;
      from_to = format!("{from}...{to}", from = prev_tag, to = current_tag);
    }

    let url = git::get_remote_url();
    let mut commits = git::get_commits(&from_to, commit_path);
    commits.sort_unstable();

    for commit in commits {
      let (commit_hash, commit_type, commit_scope, commit_title, commit_body) =
        utils::process_commit(&commit);

      let mut template = format!(
        "- [`{hash}`]({url}/commit/{hash}) ",
        hash = commit_hash,
        url = url,
      );

      if commit_scope != "" {
        template.push_str(&format!("**{scope}** ", scope = commit_scope))
      }

      template.push_str(commit_title);

      if commit_body != "" {
        template
          .push_str(&format!("\n\n{body}", body = utils::indent(commit_body)))
      }

      template.push('\n');
      println!("{}", template);
    }
  }
}
