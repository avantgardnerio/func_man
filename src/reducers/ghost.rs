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

            // it's functional, so don't mutate the RNG
            let rand = SmallRng::seed_from_u64(rand).next_u64();

            // if he hits a wall, pick a new direction
            let new_vel = if on_square
                && map_hit(&state.map, cur_map_pos + desired_vel) == MapCell::Wall
            {
                // start with all directions
                let all_dirs: Vec<Vec2d<i32>> = [[1, 0], [0, 1], [-1, 0], [0, -1]]
                    .iter()
                    .map(|d| (*d).into())
                    .collect();

                // remove invalid directions
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

                // pick a random one
                let idx = rand as usize % available_dirs.len();
                let next_dir = available_dirs[idx];
                next_dir
            } else {
                desired_vel
            };

            let new_pos = ghost.pos + new_vel;
            let final_pos = if state.pacman.dying > 0 {
                ghost.pos
            } else {
                new_pos
            };

            // if we hit a powered up pacman, die and go home
            let home_pos = if state.pacman.power > 0 && (ghost.pos - state.pacman.pos).len() < 10.0
            {
                [104, 88].into()
            } else {
                final_pos
            };

            let ghost = Ghost {
                pos: home_pos,
                vel: new_vel,
            };
            let ghosts = ghosts.into_iter().chain(once(ghost)).collect();
            (rand, ghosts)
        })
}
