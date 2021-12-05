use crate::render::window::Window;
use crate::render::display::Display;
use crate::render::window::ScreenSize;
use crate::render::writeable_display::WriteableDisplay;

pub struct CommandWindow {
    data: Vec<String>,
    display: Display
}

impl CommandWindow {
    pub fn new(size: ScreenSize) -> CommandWindow {
        CommandWindow {
            data: vec![String::new()],
            display: Display::new(size)
        }
    }
}

impl Window for CommandWindow {
    fn on_keypress(&mut self, _c: i32) -> bool {
        let writeable_display = self.display.as_writeable_mut();
        writeable_display.listen();

        let line = self.display.get_cursor_line_data().trim().to_string();

        if line.is_empty() { return true }

        // I know, I know, unsafe, but this is
        // just for personal use.
        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(line)
            .output()
            .unwrap();

        let raw_output = if output.status.success() { output.stdout } else { output.stderr };
        self.display.queue_write(
            &String::from_utf8(raw_output)
                .expect("invalid string encoding"),
            (self.display.lines() - 1, 0));

        true
    }

    fn on_activate(&mut self) {
        self.display.queue_write_buffer(&self.data);
    }

    fn data(&self) -> &Vec<String> { &self.data }

    fn start_position(&self) -> usize { 0 }
    fn set_start_position(&mut self, _new_position: usize) { }

    fn display(&self) -> &Display { &self.display }
    fn display_mut(&mut self) -> &mut Display { &mut self.display }
}
