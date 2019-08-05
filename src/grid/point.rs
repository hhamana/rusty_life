pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

impl Cell {
    pub fn new(x: usize, y: usize, alive: bool) -> Cell {
        let cell = Cell {
            x,
            y,
            alive
        };
        cell
    }

    pub fn next_cell_generation(self, live_cell: u32) -> Cell {
        /* 
        example for 11; [{0,0,true},{1,0,true},{2,0,true},{0,1,true},{0,2,true},{2,1,true},{2,2,true},{1,2,true}]
        00 01 02 03 04
        10 11 12 13 14
        20 21 22 23 24
        30 31 32 33 34
        40 41 42 43 44
        */
        let lives_on = rules(live_cell, self.alive);
        let cell = Cell::new(self.x, self.y, lives_on);
        cell
    }
}

fn rules(live_neighbors: u32, alive: bool) -> bool {
    if alive == false {
        if live_neighbors == 3 {
            true
        } else {
            false
        }
    } else {
        if live_neighbors < 2 {
            false
        } else if live_neighbors > 3 {
            false
        } else {
            true
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn create_cell() {
        let cell = Cell::new(5485,452,true);
        assert_eq!(cell.x, 5485);
        assert_eq!(cell.y, 452);
        assert_eq!(cell.alive, true);
    }

    #[test]
    fn dies_of_loneliness() {
        let cell = Cell::new(1,1,true);
        // let test_cells = vec![Cell::new(0,0,false), Cell::new(1,0,false), Cell::new(2,0,false), Cell::new(0,1,false), Cell::new(0,2,false), Cell::new(2,1,false), Cell::new(1,2,false)];Z
        let result = cell.next_cell_generation(1);
        assert_eq!(result.alive, false);
    }

    #[test]
    fn dies_of_overcrowding() {
        let cell = Cell::new(1,1,true);
        // let test_cells = vec![Cell::new(0,0,true), Cell::new(1,0,true), Cell::new(2,0,true), Cell::new(0,1,true), Cell::new(0,2,false), Cell::new(2,1,false), Cell::new(1,2,false)];
        let result = cell.next_cell_generation(5);
        assert_eq!(result.alive, false);
    }

    #[test]
    fn survives() {
        let cell = Cell::new(1,1,true);
        // let test_cells = vec![Cell::new(0,0,true), Cell::new(1,0,false), Cell::new(2,0,false), Cell::new(0,1,false), Cell::new(0,2,true), Cell::new(2,1,false), Cell::new(1,2,false)];
        let result = cell.next_cell_generation(2);
        assert_eq!(result.alive, true);
    }
    #[test]
    fn reborns() {
        let cell = Cell::new(1,1,true);
        // let test_cells = vec![Cell::new(0,0,true), Cell::new(1,0,false), Cell::new(2,0,false), Cell::new(0,1,false), Cell::new(0,2,true), Cell::new(2,1,false), Cell::new(1,2,false)];
        let result = cell.next_cell_generation(3);
        assert_eq!(result.alive, true);
    }
}