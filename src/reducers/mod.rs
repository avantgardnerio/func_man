use crate::{GameState, Movement, PacMan};
use winit::event::VirtualKeyCode;

pub const MS_PER_UPDATE: f32 = 1.0;

pub fn tick(state: &GameState, last_key: Option<VirtualKeyCode>) -> GameState {
    const UPDATE_PER_TICK: f32 = 1.0 / 30.0;
    let new_vel: Movement = match last_key {
        Some(VirtualKeyCode::Left) => Movement { x: -1, y: 0 },
        Some(VirtualKeyCode::Up) => Movement { x: 0, y: -1 },
        Some(VirtualKeyCode::Right) => Movement { x: 1, y: 0 },
        Some(VirtualKeyCode::Down) => Movement { x: 0, y: 1 },
        _ => state.pacman.vel,
    };
    GameState {
        pacman: PacMan {
            vel: new_vel,
            pos: crate::Position {
                x: state.pacman.pos.x + new_vel.x as f32 * UPDATE_PER_TICK,
                y: state.pacman.pos.y + new_vel.y as f32 * UPDATE_PER_TICK,
            },
        },
        time: state.time + 1,
        ..state.clone()
    }
}
