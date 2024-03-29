#![allow(dead_code)]
use std::path::Path;

pub enum FileState {
    Invalid,
    Unknown,
    Modified,
    Deleted,
    Added,
    Staged,
    Untracked
}

fn parse_status(path: &str) -> (char, char) {
    let state = &path[..3];

    let first  = state.chars().next().unwrap();
    let second = state.chars().nth(1).unwrap();

    (first, second)
}

pub fn is_in_worktree(path: &str) -> bool {
    let (first, _) = parse_status(path);
    first != ' ' && first != '?'
}

pub fn is_file_modified(path: &str) -> bool {
    let (first, second) = parse_status(path);

    first == 'M'
        || second == 'M'
        || first  == '?'
        || second == 'D'
        || second == 'T'
        || second == 'R'
        || second == 'C'

}

static ALLOWED_GIT_PATH_START: [char; 8] = [' ', 'M', 'T', 'A', 'D', 'R', 'C', '?'];

pub fn parse_file_state(path: &str) -> FileState {
    // https://git-scm.com/docs/git-status
    if path.len() < 3 {
        return FileState::Unknown
    }

    // unwrap _should_ be safe here because the length was already
    // checked above.
    let first = &path.chars().next().unwrap();
    let third = &path.chars().nth(2).unwrap();
    if !ALLOWED_GIT_PATH_START.contains(first) || third != &' ' {
        return FileState::Unknown
    }

    let state = &path[..3];
    let (first, second) = parse_status(path);

    if second == 'M' || first == 'A' {
        FileState::Modified
    } else if first == 'M' || first == 'A' {
        FileState::Staged
    } else if first == 'D' || second == 'D' {
        FileState::Deleted
    } else if state.starts_with("??") {
        FileState::Untracked
    } else {
        FileState::Unknown
    }
}

pub fn is_ignored(path: &Path) -> bool {
    let path_str = path.to_str().unwrap();
    let output = run(vec!["check-ignore", path_str]);

    !output.is_empty()
}

pub fn current_branch() -> String {
    run(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .first()
        .unwrap()
        .clone()
}

pub fn last_origin_commit_hash() -> String {
    run(vec!["rev-parse", &format!("origin/{}", &current_branch())])
        .first()
        .unwrap()
        .clone()
}

pub fn last_commit_hash() -> String {
    run(vec!["rev-parse", &current_branch()])
        .first()
        .unwrap()
        .clone()
}

pub fn last_origin_commit() -> String {
    run(vec!["log", "-1", "--oneline", "--no-decorate", &format!("origin/{}", &current_branch())])
        .first()
        .unwrap()
        .clone()
}

pub fn last_commit() -> String {
    run(vec!["log", "-1", "--oneline", "--no-decorate"])
        .first()
        .unwrap()
        .clone()
}

pub fn origin_head_branch() -> String {
    run(vec!["show", "-s", "--pretty=%d", &format!("origin/{}", &current_branch())])
        .first()
        .unwrap()
        .clone()

}

pub fn head_branch() -> String {
    run(vec!["show", "-s", "--pretty=%d", "HEAD"])
        .first()
        .unwrap()
        .clone()
}

pub fn status() -> Vec<String> {
    run(vec!["status", "-s"])
}

pub fn diff_file(path: &str) -> Vec<String> {
    run(vec!["--no-pager", "diff", path])
}

pub fn diff_commit(commit_hash: &str) -> Vec<String> {
    run(vec!["--no-pager", "diff", &(commit_hash.to_owned() + "^!")])
}

pub fn add_file(path: &str) {
    run(vec!["add", path]);
}

pub fn unstage_file(path: &str) {
    run(vec!["reset", path]);
}

pub fn push(push_args: Option<Vec<&str>>) -> Vec<String> {
    let mut args = vec!["push"];

    if let Some(process_args) = push_args {
        args.extend(process_args);
    }

    // TODO: what if it's not origin?
    // TODO: what if I want to choose branch?
    let current_branch = current_branch();
    args.extend(vec!["origin", &current_branch]);

    run(args)
}

pub fn commit(commit_args: Option<Vec<&str>>) -> Vec<String> {
    let mut args = vec!["commit"];

    if let Some(process_args) = commit_args {
        args.extend(process_args);
    }

    let output = std::process::Command::new("git")
        .args(args)
        .spawn()
        .unwrap()
        .wait_with_output()
        .expect("failed to execute process");

    output_lines(output)
}

pub fn branch() -> Vec<String> {
    run(vec!["--no-pager", "branch"])
}

pub fn checkout_branch(branch_name: &str) -> Vec<String> {
    run(vec!["checkout", branch_name])
}

pub fn checkout_file(file_path: &str) -> Vec<String> {
    run(vec!["checkout", file_path])
}

pub fn delete_branch(branch_name: &str) -> Vec<String> {
    run(vec!["branch", "-D", branch_name])
}

// This func should actually be called branch,
// as in, the verb.
pub fn create_branch(branch_name: &str) -> Vec<String> {
    run(vec!["branch", branch_name])
}

pub fn reset(commit_hash: &str, mode: &str) -> Vec<String> {
    run(vec!["reset", mode, commit_hash])
}

pub fn show(commit_hash: &str) -> Vec<String> {
    run(vec!["--no-pager", "show", commit_hash])
}

pub fn log(max_count: Option<u32>) -> Vec<String> {
    let mut args = vec![
        "--no-pager",
        "log",
        "--graph",
        "--oneline",
        "--decorate",
        "--remotes",
        "--branches"
    ];

    let max_count_arg;

    if let Some(max) = max_count {
        max_count_arg = format!("--max-count={}", max);
        args.push(&max_count_arg);
    }

    run(args)
}

pub fn run(args: Vec<&str>) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    output_lines(output)
}

fn output_lines(output: std::process::Output) -> Vec<String> {
    let descriptor = if output.stdout.is_empty() { output.stderr } else { output.stdout };
    let output_str = String::from_utf8(descriptor).expect("invalid string encoding");

    if output_str.is_empty() {
        vec![]
    } else {
        output_str.split('\n').map(str::to_owned).collect()
    }
}
