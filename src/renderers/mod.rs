mod floor;
mod ghost;
mod pacman;
mod pellet;
mod powerup;
mod wall;

use crate::reducers::{GameState, MapCell};
use crate::renderers::floor::render_floor;
use crate::renderers::ghost::render_ghost;
use crate::renderers::pacman::render_pacman;
use crate::renderers::pellet::render_pellet;
use crate::renderers::powerup::render_powerup;
use crate::renderers::wall::render_wall;
use femtovg::renderer::OpenGl;
use femtovg::Canvas;

pub fn render_scene(canvas: &mut Canvas<OpenGl>, state: &GameState) {
    // map
    for y in 0..state.map.len() as u32 {
        for x in 0..state.map.len() as u32 {
            match state.map[y as usize][x as usize] {
                MapCell::Wall => render_wall(canvas, x as f32, y as f32),
                MapCell::Pellet => render_pellet(canvas, x as f32, y as f32),
                MapCell::GhostStart => render_floor(),
                MapCell::Floor => render_floor(),
                MapCell::Powerup => render_powerup(canvas, x as f32, y as f32, state.time as f32),
            };
        }
    }

    // ghosts
    state.ghosts.iter().for_each(|ghost| {
        render_ghost(
            canvas,
            state.pacman.power,
            ghost.pos.0 as f32,
            ghost.pos.1 as f32,
        );
    });

    // pacman
    render_pacman(
        canvas,
        state.pacman.dying,
        state.pacman.pos.0 as f32,
        state.pacman.pos.1 as f32,
    );
}
