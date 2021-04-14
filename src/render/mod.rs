pub mod renderer;
pub mod window;
pub mod ascii_table;

pub trait Render {
    fn render(&mut self);
}
