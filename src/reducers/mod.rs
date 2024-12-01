use crate::{GameState, PacMan};
use winit::event::VirtualKeyCode;

pub const MS_PER_UPDATE: f32 = 1.0;

pub fn tick(state: &GameState, last_key: Option<VirtualKeyCode>) -> GameState {
    const UPDATE_PER_TICK: f32 = 1.0 / 30.0;
    let new_vel: [f32; 2] = match last_key {
        Some(VirtualKeyCode::Left) => [-1.0, 0.0], // TODO use Position
        Some(VirtualKeyCode::Up) => [0.0, -1.0],
        Some(VirtualKeyCode::Right) => [1.0, 0.0],
        Some(VirtualKeyCode::Down) => [0.0, 1.0],
        _ => state.pacman.vel,
    };
    GameState {
        pacman: PacMan {
            vel: new_vel,
            pos: crate::Position {
                x: state.pacman.pos.x + new_vel[0] * UPDATE_PER_TICK,
                y: state.pacman.pos.y + new_vel[1] * UPDATE_PER_TICK,
            },
        },
        time: state.time + 1,
        ..state.clone()
    }
}
