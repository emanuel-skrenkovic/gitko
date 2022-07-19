use gitko_render::{Component, KeyHandlers, Line, Window};
use gitko_common::ascii_table::{KEY_J_LOWER, KEY_K_LOWER};

pub struct CommandWindow {}

impl Component<CommandWindow> for CommandWindow {
    fn on_render(&mut self, window: &mut Window) -> bool {
        window.listen();

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

        let mut lines = window.lines();

        lines.push(
            Line::from_string(
                String::from_utf8(raw_output)
                         .expect("invalid string encoding"),
                None
            )
        );

        window.set_lines(lines);

        true
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommandWindow>) {
        handlers.remove(&KEY_J_LOWER);
        handlers.remove(&KEY_K_LOWER);
    }
}
