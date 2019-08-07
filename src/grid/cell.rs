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
