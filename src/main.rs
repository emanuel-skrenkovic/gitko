use std::process::Command;

fn main() {
    Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .spawn()
        .expect("failed to execute process");
}
