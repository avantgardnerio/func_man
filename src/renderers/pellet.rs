use crate::PX_PER_CELL;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub fn render_pellet(canvas: &mut Canvas<OpenGl>, x: f32, y: f32) {
    let radius = (PX_PER_CELL / 4.0).abs();
    let x = x * PX_PER_CELL + PX_PER_CELL / 2.0;
    let y = y * PX_PER_CELL + PX_PER_CELL / 2.0;

    let mut path = Path::new();
    path.circle(x, y, radius);
    canvas.fill_path(&path, &Paint::color(Color::rgba(0xFF, 0xFF, 0xFF, 0xFF)));
}
