use crate::math::Vec2d;

pub fn map_hit(map: [[u8; 28]; 28], pos: Vec2d<i32>) -> u8 {
    map[pos.1 as usize][pos.0 as usize]
}

pub fn map_clone_and_set(map: [[u8; 28]; 28], pos: Vec2d<i32>, val: u8) -> [[u8; 28]; 28] {
    let map_vec: Vec<[u8; 28]> = map
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let row_vec: Vec<u8> = row
                .iter()
                .enumerate()
                .map(|(cell_idx, cell)| {
                    if pos == [cell_idx as i32, row_idx as i32].into() {
                        val
                    } else {
                        *cell
                    }
                })
                .collect();
            let mut row = [0u8; 28];
            row.copy_from_slice(row_vec.as_slice());
            row
        })
        .collect();
    let mut map = [[0u8; 28]; 28];
    map.copy_from_slice(map_vec.as_slice());
    map
}
