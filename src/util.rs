use crate::math::Vec2d;

pub fn map_hit(map: [[u8; 28]; 28], pos: Vec2d<i32>) -> u8 {
    map[pos.1 as usize][pos.0 as usize]
}
