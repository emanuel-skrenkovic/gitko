pub mod commands;

pub trait GitRunner {
    fn run_git_command(&mut self);
}
