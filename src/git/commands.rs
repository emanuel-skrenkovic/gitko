pub fn status() -> Vec<String> {
    run(vec!["status", "-s"])
}

pub fn diff_file(path: &str) -> Vec<String> {
    run(vec!["--no-pager", "diff", path])
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

fn run(args: Vec<&str>) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).expect("invalid string encoding");

    output_str.split('\n').map(str::to_string).collect()
}
