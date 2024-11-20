use crate::INITIAL_MAP;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Paint, Path};
use std::arch::aarch64::vabs_f32;

pub fn render_powerup(canvas: &mut Canvas<OpenGl>, x: f32, y: f32, time: f32) {
    let px_per_cell = (canvas.width() as f32 / INITIAL_MAP.len() as f32).floor();
    let radius = ((time % (px_per_cell)) - px_per_cell / 2.0).abs();
    let x = x * px_per_cell + px_per_cell / 2.0;
    let y = y * px_per_cell + px_per_cell / 2.0;

    let mut path = Path::new();
    path.circle(x, y, radius);
    canvas.fill_path(&path, &Paint::color(Color::rgba(0xFF, 0xFF, 0xFF, 0xFF)));
}
