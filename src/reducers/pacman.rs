use super::{GameState, Movement};
use crate::reducers::PacMan;
use crate::PX_PER_CELL;
use winit::event::VirtualKeyCode;

pub fn pacman(state: &GameState, last_key: Option<VirtualKeyCode>) -> PacMan {
    const UPDATE_PER_TICK: f32 = 1.0 / 30.0;

    let desired_vel: Movement = match last_key {
        Some(VirtualKeyCode::Left) => Movement { x: -1, y: 0 },
        Some(VirtualKeyCode::Up) => Movement { x: 0, y: -1 },
        Some(VirtualKeyCode::Right) => Movement { x: 1, y: 0 },
        Some(VirtualKeyCode::Down) => Movement { x: 0, y: 1 },
        _ => state.pacman.vel,
    };
    let desired_pos = state.pacman.pos + desired_vel * UPDATE_PER_TICK;

    let map_pos = desired_pos / PX_PER_CELL as i32;
    let vel = if state.map[map_pos.y as usize][map_pos.x as usize] == 0 {
        Movement { x: 0, y: 0 } // hit a wall
    } else {
        desired_vel // move
    };
    let pos = state.pacman.pos + vel * UPDATE_PER_TICK;

    PacMan { vel, pos }
}
