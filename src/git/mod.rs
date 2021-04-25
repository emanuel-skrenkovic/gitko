pub trait GitRunner {
    fn run_git_command(&mut self);
}

pub fn run_status_command() -> Vec<String> {
    run(vec!["status".to_string(), "-s".to_string()])
}

pub fn run_diff_command(path: &str) -> Vec<String> {
    run(vec![
        "--no-pager".to_string(),
        "diff".to_string(),
        path.to_string(),
    ])
}

pub fn run_add_command(path: &str) {
    run(vec!["add".to_string(), path.to_string()]);
}

pub fn unstage_file(path: &str) {
    run(vec![
        "restore".to_string(),
        "--staged".to_string(),
        path.to_string(),
    ]);
}

/*
pub fn commit(message: Vec<str>) {

}
*/

fn run(args: Vec<String>) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).expect("invalid string encoding");

    output_str.split('\n').map(str::to_string).collect()
}
