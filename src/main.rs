mod git;
mod render;

use crate::render::Render;

fn main() {
    let mut renderer = render::renderer::Renderer::new(800, 600);

    renderer.render()
}
