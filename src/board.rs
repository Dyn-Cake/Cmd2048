
use crate::location::Location;

pub struct Board {
    pub grid: Vec<Vec<u8>>,
}
impl Board {
    pub fn new(height: u8, length: u8) -> Board {
        
        Board {
            grid: vec![vec![0; length as usize]; height as usize],
        }
        
    }
    fn gravity(&self, mut current_list: Vec<Location>) {
        let last_available = 0;
        for loc in current_list {
            if loc.getNum(self) == 0 {
                current_list[last_available] = loc;
            }
        }
    }
    fn up(& self) {
        for (i, y) in self.grid.iter().enumerate() {
            let mut current_list: Vec<Location> = Vec::new();
            for x in self.grid.iter().rev().enumerate() {
                current_list[current_list.len() + 1] = Location::new(x.0 as u8, i as u8);

            }
        }
    }
    
}
