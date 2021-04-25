pub const GREEN_TEXT_PAIR: i16 = 1;
pub const RED_TEXT_PAIR: i16 = 2;

pub trait ColoredWindow {
    fn apply_colors(&self, line: &str);
}
