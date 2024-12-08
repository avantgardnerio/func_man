mod ghost;
mod map;
mod pacman;

use crate::math::Vec2d;
use crate::reducers::ghost::ghosts;
use crate::reducers::map::map_reducer;
use crate::reducers::pacman::pacman;
use crate::reducers::MapCell::{Floor, GhostStart, Pellet, Powerup, Wall};
use serde_repr::{Deserialize_repr, Serialize_repr};
use winit::event::VirtualKeyCode;

pub fn tick(state: &GameState, last_key: Option<VirtualKeyCode>) -> GameState {
    let (rand, ghosts) = ghosts(state);
    let pacman = pacman(state, last_key);
    if pacman.dying > 100 {
        initial_state()
    } else {
        GameState {
            ghosts,
            pacman,
            time: state.time + 1,
            rand,
            map: map_reducer(state),
            ..state.clone()
        }
    }
}

#[derive(Clone)]
pub struct PacMan {
    pub pos: Vec2d<i32>,
    pub vel: Vec2d<i32>,
    pub power: i32,
    pub dying: i32,
}

#[derive(Clone)]
pub struct Ghost {
    pub pos: Vec2d<i32>,
    pub vel: Vec2d<i32>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum MapCell {
    Wall = 0,
    Pellet = 1,
    GhostStart = 2,
    Floor = 3,
    Powerup = 4,
}

impl From<u8> for MapCell {
    fn from(value: u8) -> Self {
        match value {
            0 => Wall,
            1 => Pellet,
            2 => GhostStart,
            3 => Floor,
            4 => Powerup,
            _ => unimplemented!("Unknown map cell"),
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    pub pacman: PacMan,
    pub ghosts: Vec<Ghost>,
    pub time: usize,
    pub rand: u64,
    pub map: Vec<Vec<MapCell>>,
}

pub fn initial_state() -> GameState {
    let map = INITIAL_MAP
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| MapCell::from(*cell))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    GameState {
        pacman: PacMan {
            pos: [104, 160].into(),
            vel: [0, 0].into(),
            power: 0,
            dying: 0,
        },
        ghosts: vec![
            Ghost {
                pos: [104, 88].into(),
                vel: [0, -1].into(),
            },
            Ghost {
                pos: [112, 88].into(),
                vel: [0, -1].into(),
            },
            Ghost {
                pos: [104, 96].into(),
                vel: [0, -1].into(),
            },
            Ghost {
                pos: [112, 96].into(),
                vel: [0, -1].into(),
            },
        ],
        time: 0,
        rand: 42,
        map,
    }
}

#[rustfmt::skip]
pub const INITIAL_MAP: [[u8; 28]; 28] = [
    [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [ 0, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 2, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 2, 2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, ],
    [ 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, ],
    [ 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, ],
    [ 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, ],
    [ 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, ],
    [ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, ],
    [ 0, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 0, ],
    [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
];
