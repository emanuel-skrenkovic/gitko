pub mod renderer;
pub mod window;

pub trait Render {
    fn render(&mut self);
}
