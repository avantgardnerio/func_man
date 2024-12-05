use crate::math::Vec2d;
use crate::reducers::MapCell;

pub fn map_hit(map: &Vec<Vec<MapCell>>, pos: Vec2d<i32>) -> MapCell {
    map[pos.1 as usize][pos.0 as usize]
}

pub fn map_clone_and_set(
    map: &Vec<Vec<MapCell>>,
    pos: Vec2d<i32>,
    val: MapCell,
) -> Vec<Vec<MapCell>> {
    map.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(cell_idx, cell)| {
                    if pos == [cell_idx as i32, row_idx as i32].into() {
                        val
                    } else {
                        cell.clone()
                    }
                })
                .collect()
        })
        .collect()
}
