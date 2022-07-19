use crate::screen;
use crate::gitko::input_window::InputWindow;
use gitko_render::{Position, ScreenSize, KeyHandlers, Window, Component, Renderer};

use gitko_common::ascii_table::{KEY_ETB, KEY_FORWARD_SLASH, KEY_N_LOWER, KEY_N_UPPER};

pub trait SearchableComponent<T: SearchableComponent<T> + Component<T>>: Component<T> {
    fn term(&self) -> String;
    fn set_term(&mut self, term: String);

    fn search_init(&mut self, window: &mut Window) -> bool {
        self.set_term("".to_owned());
        window.show_cursor(true);

        let mut search_window = InputWindow::new();

        Renderer::new(
            &mut search_window,
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 },
            screen()
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

    fn search_stop(&mut self, window: &mut Window) -> bool {
        self.set_term("".to_owned());

        // TODO: what if the window always shows cursor?
        window.show_cursor(false);
        true
    }
}

pub fn register_search_handlers<T: SearchableComponent<T>>(handlers: &mut KeyHandlers<T>) {
    handlers.insert(KEY_N_LOWER, SearchableComponent::next_search_result);
    handlers.insert(KEY_N_UPPER, SearchableComponent::prev_search_result);
    handlers.insert(KEY_FORWARD_SLASH, SearchableComponent::search_init);
    handlers.insert(KEY_ETB, SearchableComponent::search_stop);
}
