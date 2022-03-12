use crate::ascii_table::*;
use crate::render::{Component, KeyHandlers, Window};

pub struct PromptWindow<TYes: Fn(), TNo: Fn()> {
    message: String,
    on_yes: TYes,
    on_no: TNo
}

impl<TYes: Fn(), TNo: Fn()> PromptWindow<TYes, TNo> {
    pub fn new(message: &str, on_yes: TYes, on_no: TNo) -> PromptWindow<TYes, TNo> {
        PromptWindow {
            message: message.to_owned(),
            on_yes,
            on_no
        }
    }

    fn yes(&mut self, _window: &mut Window) -> bool {
        (self.on_yes)();
        false
    }

    fn no(&mut self, _window: &mut Window) -> bool {
        (self.on_no)();
        false
    }
}

impl<TYes: Fn(), TNo: Fn()> Component<PromptWindow<TYes, TNo>> for PromptWindow<TYes, TNo> {
    fn on_start(&mut self, window: &mut Window) {
        window.data = vec![self.message.clone()];
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<PromptWindow<TYes, TNo>>) {
        handlers.insert(KEY_Y_LOWER, PromptWindow::yes);
        handlers.insert(KEY_N_LOWER, PromptWindow::no);
    }
}
