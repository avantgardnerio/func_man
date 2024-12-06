use crate::math::Vec2d;
use crate::reducers::{GameState, Ghost, MapCell, PacMan};
use crate::util::map_hit;
use crate::PX_PER_CELL;
use winit::event::VirtualKeyCode;

pub fn ghosts(state: &GameState) -> Vec<Ghost> {
    state
        .ghosts
        .iter()
        .map(|ghost| {
            let desired_vel = ghost.vel;
            let cur_map_pos = state.pacman.pos / PX_PER_CELL as i32;
            let on_square = (state.pacman.pos % PX_PER_CELL as i32).len_sq() == 0;

            let new_pos = ghost.pos + desired_vel;

            Ghost {
                pos: new_pos,
                vel: desired_vel,
            }
        })
        .collect()
}
