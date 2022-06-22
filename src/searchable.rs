use crate::ascii_table::*;
use crate::gitko::input_window::InputWindow;
use crate::render::{Position, ScreenSize, KeyHandlers, Window, Component, Renderer};

pub trait SearchableComponent<T: SearchableComponent<T> + Component<T>>: Component<T> {
    fn term(&self) -> String;
    fn set_term(&mut self, term: String);

    fn search_init(&mut self, window: &mut Window) -> bool {
        self.set_term("".to_owned());
        let mut search_window = InputWindow::new();

        Renderer::new(
            &mut search_window,
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.set_term(search_window.text);
        window.move_next(&self.term());

        true
    }

    fn next_search_result(&mut self, window: &mut Window) -> bool {
        window.move_next(&self.term());
        true
    }

    fn prev_search_result(&mut self, window: &mut Window) -> bool {
        window.move_prev(&self.term());
        true
    }
}

pub fn register_search_handlers<T: SearchableComponent<T>>(handlers: &mut KeyHandlers<T>) {
    handlers.insert(KEY_N_LOWER, SearchableComponent::next_search_result);
    handlers.insert(KEY_N_UPPER, SearchableComponent::prev_search_result);
    handlers.insert(KEY_FORWARD_SLASH, SearchableComponent::search_init);
}
