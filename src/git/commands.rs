#![allow(dead_code)]

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
    run(vec!["restore", "--staged", path]);
}

pub fn commit(message: &str) {
    run(vec!["commit", "-m", message]);
}

pub fn branch() -> Vec<String> {
    run (vec!["--no-pager", "branch"])
}

pub fn checkout_branch(branch_name: &str) -> Vec<String> {
    run (vec!["checkout", branch_name])
}

pub fn delete_branch(branch_name: &str) -> Vec<String> {
    run(vec!["branch", "-D", branch_name])
}

pub fn log(max_count: Option<u32>) -> Vec<String> {
    let mut args = vec!["--no-pager", "log", "--graph", "--oneline", "--decorate"];

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

    let output_str = String::from_utf8(output.stdout).expect("invalid string encoding");

    output_str.split('\n').map(str::to_string).collect()
}
