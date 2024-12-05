use crate::reducers::{GameState, MapCell};
use crate::util::map_clone_and_set;
use crate::PX_PER_CELL;

pub fn map_reducer(state: &GameState) -> Vec<Vec<MapCell>> {
    let cur_map_pos = state.pacman.pos / PX_PER_CELL as i32;
    let on_square = (state.pacman.pos % PX_PER_CELL as i32).len_sq() == 0;
    if on_square {
        map_clone_and_set(&state.map, cur_map_pos, MapCell::Floor)
    } else {
        state.map.clone()
    }
}
