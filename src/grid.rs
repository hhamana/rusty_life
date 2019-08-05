mod point;

pub struct Grid {
    width: usize,
    height: usize,
    points: Vec<Vec<point::Cell>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut horizontal = Vec::new();
        for x in 0..width {
            let mut vertical = Vec::new();
            for y in 0..height {
                vertical.push(point::Cell::new(x, y, false))
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

    pub fn tick(&self) -> Grid {
        let mut horizontal = Vec::new();
        // mulithread that part
        for x in self.points {
            let mut vertical = Vec::new();
            for point in x {
                let neighbors = self.get_live_neighboring_points(&point);
                let cell = point.next_cell_generation(neighbors);
                vertical.push(cell);
            }
            // to here
            horizontal.push(vertical)
        }
        let new_grid = Grid {
            width: self.width,
            height: self.height,
            points: horizontal
        };
        new_grid
    }

    fn get_live_neighboring_points(&self, point: &point::Cell) -> u32 {
        // +2 for max boundary because range is exclusive at the end
        let xmin = point.x.saturating_sub(1);
        let xmax = {
            if point.x+2 > self.width { self.width } else { point.x+2 }
        };
        let ymin = point.y.saturating_sub(1);
        let ymax = {
            if point.y+2 > self.height { self.height } else { point.y+2 }
        };
        let mut live_cells = 0;

        for x_axis in xmin..xmax {
            for y_axis in ymin..ymax {
                println!("{} {}", x_axis, y_axis);
                match (x_axis == point.x, y_axis== point.y) {
                    (true,true) => println!("Self match"),
                    (_,_) => {
                        if self.points[x_axis][y_axis].alive {
                            live_cells += 1;
                        }
                    }
                }
            }
        }
        live_cells

    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
// 	fn new_grid(){
//         let grid = Grid::new(256, 256);
//         assert_eq!(grid.width, 256);
//         assert_eq!(grid.height, 256);
//         assert_eq!(grid.points.len(), 256);
//         assert_eq!(grid.points[255].len(), 256);
//     }

//     #[test]
// 	fn new_grid_is_made_of_points(){
//         let grid = Grid::new(256, 256);
//         assert_eq!(grid.points[4][5].x, 4);
//         assert_eq!(grid.points[4][5].y, 5);
//         assert_eq!(grid.points[4][5].alive, false);
//     }

//     #[test]
// 	fn get_neighboring_points_gets_up_to_8_points(){
//         let grid = Grid::new(256, 256);
//         let result = get_live_neighboring_points(&grid, &grid.points[50][30]);
//         assert_eq!(result.len(), 8);
//     }

//     #[test]
// 	fn get_neighboring_points_gets_points(){
//         let grid = Grid::new(256, 256);
//         let result = get_neighboring_points(&grid, &grid.points[50][30]);
//         assert_eq!(result[0].x, 49);
//         assert_eq!(result[0].y, 29);
//         assert_eq!(result[7].x, 51);
//         assert_eq!(result[7].y, 31);
//     }
    
//     #[test]
// 	fn get_neighboring_points_stays_positive(){
//         let grid = Grid::new(256, 256);
//         let result = get_neighboring_points(&grid, &grid.points[0][30]);
//         assert_eq!(result.len(), 5);
//         assert_eq!(result[0].x, 0);
//         assert_eq!(result[0].y, 29);
//         assert_eq!(result[4].x, 1);
//         assert_eq!(result[4].y, 31);
//     }
    
//     #[test]
// 	fn get_neighboring_points_stays_within_boundaries(){
//         let grid = Grid::new(256, 256);
//         let result = get_neighboring_points(&grid, &grid.points[255][255]);
//         assert_eq!(result.len(), 3);
//         assert_eq!(result[0].x, 254);
//         assert_eq!(result[0].y, 254);
//         assert_eq!(result[1].x, 254);
//         assert_eq!(result[1].y, 255);
//         assert_eq!(result[2].x, 255);
//         assert_eq!(result[2].y, 254);
//     }
    
//     #[test]
// 	fn tick(){
//         let grid = Grid::new(16, 16);
//         let result = tick();
//         assert_eq!(result.points.len(), 16);
//     }
    
// }