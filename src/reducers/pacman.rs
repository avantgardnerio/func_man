use crate::math::Vec2d;
use crate::reducers::{GameState, MapCell, PacMan};
use crate::util::map_hit;
use crate::PX_PER_CELL;
use winit::event::VirtualKeyCode;

pub fn pacman(state: &GameState, last_key: Option<VirtualKeyCode>) -> PacMan {
    // If pacman is exactly on a tile, and the tile in the direction of the last keypress is free, change direction
    // Otherwise, retain existing velocity
    let desired_vel: Vec2d<i32> = match last_key {
        Some(VirtualKeyCode::Left) => [-1, 0].into(),
        Some(VirtualKeyCode::Up) => [0, -1].into(),
        Some(VirtualKeyCode::Right) => [1, 0].into(),
        Some(VirtualKeyCode::Down) => [0, 1].into(),
        _ => state.pacman.vel,
    };
    let cur_map_pos = state.pacman.pos / PX_PER_CELL as i32;
    let on_square = (state.pacman.pos % PX_PER_CELL as i32).len_sq() == 0;

    let desired_map = map_hit(&state.map, cur_map_pos + desired_vel);
    let new_vel = if on_square && (desired_map == MapCell::Floor || desired_map == MapCell::Pellet)
    {
        desired_vel
    } else {
        state.pacman.vel
    };

    // Move pacman to the next position, unless that position collides with a wall
    let next_pos = state.pacman.pos + new_vel;
    let new_map = map_hit(&state.map, cur_map_pos + new_vel);
    let new_pos = if on_square && (new_map == MapCell::Wall || new_map == MapCell::GhostStart) {
        state.pacman.pos
    } else {
        next_pos
    };

    // Update and return the state
    PacMan {
        vel: new_vel,
        pos: new_pos,
    }
}
