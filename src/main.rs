mod git;
mod render;
mod num;

use crate::render::Render;

fn main() {
    let mut renderer = render::renderer::Renderer::new(800, 600);

    renderer.render()
}
