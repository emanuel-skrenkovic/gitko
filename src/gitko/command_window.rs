use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Window, WriteableWindow};

pub struct CommandWindow {}

impl CommandWindow {
    pub fn new() -> CommandWindow {
        CommandWindow { }
    }

    // lol
    pub fn do_nothing(&mut self, _: &mut Window) -> bool {
        true
    }
}

impl Component<CommandWindow> for CommandWindow {
    fn on_render(&mut self, window: &mut Window) -> bool {
        window.as_writeable_mut()
              .listen();

        let line = window.get_cursor_line()
                         .trim()
                         .to_owned();

        if line.is_empty() { return false; }

        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(line)
            .output()
            .unwrap();

        let raw_output = if output.status.success() {
            output.stdout
        } else {
            output.stderr
        };

        window.data.push(String::from_utf8(raw_output)
                         .expect("invalid string encoding"));

        true
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommandWindow>) {
        handlers.insert(KEY_J_LOWER, CommandWindow::do_nothing);
        handlers.insert(KEY_K_LOWER, CommandWindow::do_nothing);
    }
}
