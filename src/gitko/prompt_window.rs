use crate::render::window::{Component, Window};
use crate::render::ascii_table::*;

pub struct PromptWindow<TYes: Fn(), TNo: Fn()> {
    data: Vec<String>,
    on_yes: TYes,
    on_no: TNo
}

impl<TYes: Fn(), TNo: Fn()> PromptWindow<TYes, TNo> {
    pub fn new(message: &str, on_yes: TYes, on_no: TNo) -> PromptWindow<TYes, TNo> {
        PromptWindow {
            data: vec![message.to_string()],
            on_yes,
            on_no
        }
    }

    fn yes(&mut self, _window: &mut Window<PromptWindow<TYes, TNo>>) -> bool {
        (self.on_yes)();
        false
    }

    fn no(&mut self, _window: &mut Window<PromptWindow<TYes, TNo>>) -> bool {
        (self.on_no)();
        false
    }
}

impl<TYes: Fn(), TNo: Fn()> Component<PromptWindow<TYes, TNo>> for PromptWindow<TYes, TNo> {
    fn data(&self) -> &[String] {
        &self.data
    }

    fn register_handlers(&self, window: &mut Window<PromptWindow<TYes, TNo>>) {
        window.register_handler(KEY_Y_LOWER, PromptWindow::yes);
        window.register_handler(KEY_N_LOWER, PromptWindow::no);
    }
}
