use crate::reducers::{GameState, PacMan};
use winit::event::VirtualKeyCode;

pub fn pacman(state: &GameState, last_key: Option<VirtualKeyCode>) -> PacMan {
    let new_vel = match last_key {
        Some(VirtualKeyCode::Left) => [-1, 0],
        Some(VirtualKeyCode::Up) => [0, -1],
        Some(VirtualKeyCode::Right) => [1, 0],
        Some(VirtualKeyCode::Down) => [0, 1],
        _ => state.pacman.vel,
    };
    PacMan {
        vel: new_vel,
        pos: [
            (state.pacman.pos[0] as i32 + new_vel[0] as i32) as u32,
            (state.pacman.pos[1] as i32 + new_vel[1] as i32) as u32,
        ],
    }
}
