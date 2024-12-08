use crate::PX_PER_CELL;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub fn render_ghost(canvas: &mut Canvas<OpenGl>, weakness: i32, x: f32, y: f32) {
    let cx = x + PX_PER_CELL / 2.0;
    let cy = y + PX_PER_CELL / 2.0;
    let color = if weakness > 0 {
        Color::rgba(0xAA, 0xAA, 0xAA, 0xAA)
    } else {
        Color::rgba(0xFF, 0x00, 0x00, 0xFF)
    };

    let mut path = Path::new();
    path.circle(cx, cy, PX_PER_CELL / 2.0);
    canvas.fill_path(&path, &Paint::color(color));
}
