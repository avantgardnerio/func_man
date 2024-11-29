mod pacman;

use crate::reducers::pacman::pacman;
use crate::GameState;
use winit::event::VirtualKeyCode;

pub fn tick(state: &GameState, last_key: Option<VirtualKeyCode>) -> GameState {
    GameState {
        pacman: pacman(state, last_key),
        time: state.time + 1,
        ..state.clone()
    }
}
