use crate::render::{Component, Line, Window};

pub struct TextWindow<'text> {
    pub lines: Vec<&'text str>
}

impl <'text> Component<TextWindow<'text>> for TextWindow<'text> {
    fn on_start(&mut self, window: &mut Window) {
        window.lines = self.lines
            .iter()
            .map(|s| Line::from_string(s.to_string()))
            .collect();
    }
}
