use crate::PX_PER_CELL;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub fn render_wall(canvas: &mut Canvas<OpenGl>, x: f32, y: f32) {
    let mut path = Path::new();
    path.rect(x * PX_PER_CELL, y * PX_PER_CELL, PX_PER_CELL, PX_PER_CELL);
    canvas.fill_path(&path, &Paint::color(Color::rgba(0, 0, 0xFF, 0xFF)));
}
