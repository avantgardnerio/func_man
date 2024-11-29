use crate::math::Vec2d;
use crate::reducers::{GameState, PacMan};
use crate::util::map_hit;
use crate::PX_PER_CELL;
use winit::event::VirtualKeyCode;

pub fn pacman(state: &GameState, last_key: Option<VirtualKeyCode>) -> PacMan {
    let desired_vel: Vec2d<i32> = match last_key {
        Some(VirtualKeyCode::Left) => [-1, 0].into(),
        Some(VirtualKeyCode::Up) => [0, -1].into(),
        Some(VirtualKeyCode::Right) => [1, 0].into(),
        Some(VirtualKeyCode::Down) => [0, 1].into(),
        _ => state.pacman.vel,
    };
    let cur_map_pos = state.pacman.pos / PX_PER_CELL as i32;
    let on_square = (state.pacman.pos % PX_PER_CELL as i32).len_sq() == 0;

    let new_vel = if on_square && map_hit(state.map, cur_map_pos + desired_vel) == 1 {
        desired_vel
    } else {
        state.pacman.vel
    };

    let next_pos = state.pacman.pos + new_vel;
    let new_pos = if on_square && map_hit(state.map, cur_map_pos + new_vel) != 1 {
        state.pacman.pos
    } else {
        next_pos
    };

    PacMan {
        vel: new_vel,
        pos: new_pos,
    }
}
