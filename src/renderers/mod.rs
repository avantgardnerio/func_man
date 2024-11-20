mod floor;
mod pacman;
mod pellet;
mod powerup;
mod wall;

use crate::renderers::floor::render_floor;
use crate::renderers::pacman::render_pacman;
use crate::renderers::pellet::render_pellet;
use crate::renderers::powerup::render_powerup;
use crate::renderers::wall::render_wall;
use crate::{GameState, INITIAL_MAP};
use femtovg::renderer::OpenGl;
use femtovg::Canvas;

pub fn render_scene(canvas: &mut Canvas<OpenGl>, state: &GameState) {
    // map
    for y in 0..INITIAL_MAP.len() as u32 {
        for x in 0..INITIAL_MAP.len() as u32 {
            match INITIAL_MAP[y as usize][x as usize] {
                0 => render_wall(canvas, x, y),
                1 => render_pellet(),
                2 => render_floor(),
                3 => render_floor(),
                4 => render_powerup(canvas, x as f32, y as f32, state.time as f32),
                _ => {}
            };
        }
    }

    // pacman
    render_pacman(canvas, state.pacman.pos[0] as f32, state.pacman.pos[1] as f32);
}
