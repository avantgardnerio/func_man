use crate::INITIAL_MAP;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color};

pub fn render_wall(canvas: &mut Canvas<OpenGl>, x: u32, y: u32) {
    let px_per_cell = canvas.width() / INITIAL_MAP.len() as u32;
    let min_x = x * px_per_cell;
    let min_y = y * px_per_cell;
    canvas.clear_rect(
        min_x,
        min_y,
        px_per_cell,
        px_per_cell,
        Color::rgb(0, 0, 0xFF),
    );
}
