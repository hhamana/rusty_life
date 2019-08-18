// Cell
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

// Grid 
pub struct Grid {
    width: usize,
    height: usize,
    points: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new_empty(width: usize, height: usize) -> Grid {
        let mut horizontal = Vec::new();
        for x in 0..width {
            let mut vertical: Vec<Cell> = Vec::new();
            for y in 0..height {
                vertical.push(Cell::new(x, y, false))
            }
            horizontal.push(vertical)
        }
        let grid = Grid {
            width,
            height,
            points: horizontal
        };
        grid
    }
    pub fn new_from_seed(seed: Vec<Vec<Cell>>) -> Grid {
        let width = seed.len();
        let height = seed[0].len();
        for column in &seed {
            assert_eq!(column.len(), height)
        }
        let grid = Grid {
            width,
            height,
            points: seed
        };
        grid
    }

    pub fn tick(self) -> Grid {
        let height = self.height;
        let width = self.width;
        let mut horizontal = Vec::new();
        // mulithread that part
        for x in &self.points {
            let mut vertical = Vec::new();
            for cell in x {
                let neighbors = get_live_neighboring_points(&self, &cell);
                let new_cell = next_cell_generation(cell, neighbors);
                vertical.push(new_cell);
            }
            // to here
            horizontal.push(vertical)
        }
        let new_grid = Grid {
            width,
            height,
            points: horizontal
        };
        new_grid
    }
}

fn get_live_neighboring_points(grid: &Grid, point: &Cell) -> u32 {
    // +2 for max boundary because range is exclusive at the end, and i need the axis +1 to look over the bounds
    let xmin = point.x.saturating_sub(1);
    let xmax = {
        if point.x+2 > grid.width { grid.width } else { point.x+2 }
    };
    let ymin = point.y.saturating_sub(1);
    let ymax = {
        if point.y+2 > grid.height { grid.height } else { point.y+2 }
    };
    let mut live_cells = 0;

    for x_axis in xmin..xmax {
        for y_axis in ymin..ymax {
            println!("{} {}", x_axis, y_axis);
            match (x_axis == point.x, y_axis== point.y) {
                (true,true) => println!("Self match"),
                (_,_) => {
                    if grid.points[x_axis][y_axis].alive {
                        live_cells += 1;
                    }
                }
            }
        }
    }
    live_cells

}

pub fn next_cell_generation(cell: &Cell, live_neighbors: u32) -> Cell {
    /*
    example for 11; [{0,0,true},{1,0,true},{2,0,true},{0,1,true},{0,2,true},{2,1,true},{2,2,true},{1,2,true}]
    00 01 02 03 04
    10 11 12 13 14
    20 21 22 23 24
    30 31 32 33 34
    40 41 42 43 44
    */
    let lives_on = match cell.alive { 
        true => rules_alive(live_neighbors),
        false => rules_dead(live_neighbors)
    };
    let cell = Cell::new(cell.x, cell.y, lives_on);
    cell
}

fn rules_alive(live_neighbors: u32) -> bool {
    if live_neighbors < 2 {
        false
    } else if live_neighbors > 3 {
        false
    } else {
        true
    }
}

fn rules_dead(live_neighbors: u32) -> bool {
    if live_neighbors == 3 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_neighboring_cells_all_alive() {
        let cell_list = vec![
            vec![Cell::new(0, 0, true), Cell::new(0, 1, true), Cell::new(0, 2, true)],
            vec![Cell::new(1, 0, true), Cell::new(1, 1, true), Cell::new(1, 2, true)],
            vec![Cell::new(2, 0, true), Cell::new(2, 1, true), Cell::new(2, 2, true)]
        ];
        let test_grid = Grid {
            width: 3,
            height: 3,
            points: cell_list
        };
        let target_cell = &test_grid.points[1][1];
        let result = get_live_neighboring_points(&test_grid, target_cell);
        assert_eq!(result, 8);
    }

    #[test]
    fn count_neighboring_cells_all_dead() {
        let cell_list = vec![
            vec![Cell::new(0, 0, false), Cell::new(1, 0, false), Cell::new(2, 0, false)],
            vec![Cell::new(0, 1, false), Cell::new(1, 1, false), Cell::new(2, 1, false)],
            vec![Cell::new(0, 2, false), Cell::new(1, 2, false), Cell::new(2, 2, false)]
        ];
        let test_grid = Grid {
            width: 3,
            height: 3,
            points: cell_list
        };
        let target_cell = &test_grid.points[1][1];
        let result = get_live_neighboring_points(&test_grid, target_cell);
        assert_eq!(result, 0);
    }

    #[test]
    fn dies_of_loneliness() {
        let cell = Cell::new(1,1,true);
        let result = next_cell_generation(&cell, 1);
        assert_eq!(result.alive, false);
    }

    #[test]
    fn dies_of_overcrowding() {
        let cell = Cell::new(1,1,true);
        let result = next_cell_generation(&cell, 5);
        assert_eq!(result.alive, false);
    }

    #[test]
    fn survives() {
        let cell = Cell::new(1,1,true);
        let result = next_cell_generation(&cell, 2);
        assert_eq!(result.alive, true);
    }
    #[test]
    fn reborns() {
        let cell = Cell::new(1,1,true);
        let result = next_cell_generation(&cell, 3);
        assert_eq!(result.alive, true);
    }

    #[test]
	fn new_grid(){
        let grid = Grid::new_empty(256, 256);
        assert_eq!(grid.points.len(), 256);
        assert_eq!(grid.points[255].len(), 256);
    }

    #[test]
	fn new_grid_is_made_of_points(){
        let grid = Grid::new_empty(256, 256);
        assert_eq!(grid.points[4][5].x, 4);
        assert_eq!(grid.points[4][5].y, 5);
        assert_eq!(grid.points[4][5].alive, false);
    }
    
    #[test]
	fn new_grid_from_seed(){
        let cell_list = vec![
            vec![Cell::new(0, 0, true), Cell::new(0, 1, false), Cell::new(0, 2, true)],
            vec![Cell::new(1, 0, false), Cell::new(1, 1, true), Cell::new(1, 2, false)],
            vec![Cell::new(2, 0, true), Cell::new(2, 1, false), Cell::new(2, 2, true)]
        ];
        let grid = Grid::new_from_seed(cell_list);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.points[1][2].x, 1);
        assert_eq!(grid.points[2][0].y, 0);
        assert_eq!(grid.points[1][1].alive, true);
    }
    
}
