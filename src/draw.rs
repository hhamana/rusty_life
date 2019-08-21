use super::grid;

const CHAR_ALIVE: &'static str = "Ｏ";
const CHAR_DEAD: &'static str = "　";
const NEWLINE: &'static str = "\n";
const CLEAR: &'static str = "\x1B[2J";

pub fn to_sysout(grid: grid::Grid) {
    println!("{}", CLEAR);
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn generate_line_returns_string() { 
//         let test_cell_line = vec![grid::Cell::new(0, 0, true), grid::Cell::new(1, 0, false), grid::Cell::new(2, 0, true)];
//         let result = generate_line(&test_cell_line);
//         assert_eq!(result, "Ｏ　Ｏ")
//     }
// }