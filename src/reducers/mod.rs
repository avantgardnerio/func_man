use crate::{GameState, PacMan};
use winit::event::VirtualKeyCode;

pub fn tick(state: &GameState, last_key: Option<VirtualKeyCode>) -> GameState {
    let new_vel = match last_key {
        Some(VirtualKeyCode::Left) => [-5, 0],
        Some(VirtualKeyCode::Up) => [0, -5],
        Some(VirtualKeyCode::Right) => [5, 0],
        Some(VirtualKeyCode::Down) => [0, 5],
        _ => state.pacman.vel,
    };
    GameState {
        pacman: PacMan {
            vel: new_vel,
            pos: [
                (state.pacman.pos[0] as i32 + new_vel[0] as i32) as u32,
                (state.pacman.pos[1] as i32 + new_vel[1] as i32) as u32,
            ],
        },
        time: state.time + 1,
        ..state.clone()
    }
}
