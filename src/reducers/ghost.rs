use crate::math::Vec2d;
use crate::reducers::{GameState, Ghost, MapCell};
use crate::util::map_hit;
use crate::PX_PER_CELL;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::iter::once;

pub fn ghosts(state: &GameState) -> (u64, Vec<Ghost>) {
    state
        .ghosts
        .iter()
        .fold((state.rand, vec![]), |(rand, ghosts), ghost| {
            let desired_vel = ghost.vel;
            let cur_map_pos = ghost.pos / PX_PER_CELL as i32;
            let on_square = (ghost.pos % PX_PER_CELL as i32).len_sq() == 0;

            let rand = SmallRng::seed_from_u64(rand).next_u64();
            let new_vel = if on_square
                && map_hit(&state.map, cur_map_pos + desired_vel) == MapCell::Wall
            {
                let all_dirs: Vec<Vec2d<i32>> = [[1, 0], [0, 1], [-1, 0], [0, -1]]
                    .iter()
                    .map(|d| (*d).into())
                    .collect();
                let available_dirs: Vec<Vec2d<i32>> =
                    all_dirs.into_iter().fold(vec![], |acc, dir| {
                        let is_forward = ghost.vel == dir;
                        let is_backward = ghost.vel * -1 == dir;
                        let is_blocked = map_hit(&state.map, cur_map_pos + dir) == MapCell::Wall;
                        if is_forward || is_backward || is_blocked {
                            acc
                        } else {
                            acc.into_iter().chain(once(dir)).collect()
                        }
                    });
                let idx = rand as usize % available_dirs.len();
                let next_dir = available_dirs[idx];
                next_dir
            } else {
                desired_vel
            };

            let new_pos = ghost.pos + new_vel;

            let ghost = Ghost {
                pos: new_pos,
                vel: new_vel,
            };
            let ghosts = ghosts.into_iter().chain(once(ghost)).collect();
            (rand, ghosts)
        })
}
