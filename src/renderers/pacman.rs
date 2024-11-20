use crate::INITIAL_MAP;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};

pub fn render_pacman(canvas: &mut Canvas<OpenGl>, x: f32, y: f32) {
    let px_per_cell = canvas.width() as f32 / INITIAL_MAP.len() as f32;
    let cx = x + px_per_cell / 2.0;
    let cy = y + px_per_cell / 2.0;

    let mut path = Path::new();
    path.circle(cx, cy, px_per_cell / 2.0);
    canvas.fill_path(&path, &Paint::color(Color::rgba(0xFF, 0xFF, 0x00, 0xFF)));

}
