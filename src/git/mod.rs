pub trait GitRunner {
    fn run_git_command(&mut self);
}

pub fn run_status_command() -> Vec<String> {
    run(vec!["status".to_string(), "-s".to_string()])
}

pub fn run_diff_command(args: Option<Vec<String>>) -> Vec<String> {
    run(vec!["--no-pager".to_string(), "diff".to_string()])
}

fn run(args: Vec<String>) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).expect("invalid string encoding");

    output_str.split('\n').map(str::to_string).collect()
}
