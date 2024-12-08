use crate::PX_PER_CELL;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub fn render_pacman(canvas: &mut Canvas<OpenGl>, dying: i32, x: f32, y: f32) {
    let cx = x + PX_PER_CELL / 2.0;
    let cy = y + PX_PER_CELL / 2.0;
    let radius = 4.0 - (dying as f32 / 100.0 * 4.0);

    let mut path = Path::new();
    path.circle(cx, cy, radius);
    canvas.fill_path(&path, &Paint::color(Color::rgba(0xFF, 0xFF, 0x00, 0xFF)));
}
