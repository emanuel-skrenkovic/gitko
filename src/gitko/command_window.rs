use crate::render::writeable_display::WriteableDisplay;
use crate::render::window::{Component, Window};
use crate::render::ascii_table::*;

pub struct CommandWindow {
    data: Vec<String>,
}

impl CommandWindow {
    pub fn new() -> CommandWindow {
        CommandWindow { data: vec![String::new()] }
    }

    // lol
    pub fn do_nothing(&mut self, _: &mut Window<CommandWindow>) -> bool {
        true
    }
}

impl Component<CommandWindow> for CommandWindow {
    fn on_render(&mut self, window: &mut Window<CommandWindow>) -> bool {
        window.display
              .as_writeable_mut()
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

        self.data.push(String::from_utf8(raw_output)
                       .expect("invalid string encoding"));

        true
    }

    fn data(&self) -> &[String] {
        &self.data
    }

    fn register_handlers(&self, window: &mut Window<CommandWindow>) {
        window.register_handler(KEY_J_LOWER, CommandWindow::do_nothing);
        window.register_handler(KEY_K_LOWER, CommandWindow::do_nothing);
    }
}
