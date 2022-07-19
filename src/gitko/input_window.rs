use gitko_render::{Component, Window};

pub struct InputWindow {
    pub text: String
}

impl InputWindow {
    pub fn new() -> InputWindow {
        InputWindow { text: "".to_owned() }
    }
}

impl Component<InputWindow> for InputWindow {
    fn on_render(&mut self, window: &mut Window) -> bool {
        window.listen();

        self.text = window.get_cursor_line().trim().to_owned();

        false
    }
}
