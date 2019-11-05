use super::grid;

/// prints the points of a single grid as a string to the standard output (console)
pub fn to_sysout(grid: grid::Grid) {
    const CHAR_ALIVE: &'static str = "Ｏ";
    const CHAR_DEAD: &'static str = "　";
    const NEWLINE: &'static str = "\n";
    const CLEAR: &'static str = "\x1B[2J";
    
    println!("{}", CLEAR);
    // let mut grid_string = "";
    for line in grid.points {
        for cell in line {
            match cell.alive {
                true => print!("{}", CHAR_ALIVE),
                false => print!("{}", CHAR_DEAD),
            }
        }
        print!("{}",NEWLINE);
    }
}