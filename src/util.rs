use crate::math::Vec2d;

pub fn map_hit(map: [[u8; 28]; 28], pos: Vec2d<i32>) -> u8 {
    let x = pos.val[0] as usize;
    let y = pos.val[1] as usize;
    map[y][x]
}
