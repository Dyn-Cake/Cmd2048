use crate::board::Board;

pub struct Location {
    x_pos: u8,
    y_pos: u8,

}
impl Location {
    pub fn new(x: u8, y: u8) -> Location {
        Location { x_pos: x, y_pos: y }
    }
    pub fn getNum(&self, board: &Board) -> u8 {
        return board.grid[self.x_pos as usize][self.y_pos as usize]
    }

}
