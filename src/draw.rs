use super::grid;

const CHAR_ALIVE: &'static str = "Ｏ";
const CHAR_DEAD: &'static str = "　";

pub fn to_sysout(grid: grid::Grid) {
    for line in &grid.points {
        let line_string = generate_line(&line);
        println!("{}", line_string)
    }
}

fn generate_line(line: &Vec<grid::Cell>) -> String {
    let mut line_string: String = "".to_owned();
    for cell in line {
        match cell.alive {
            true => line_string.push_str(CHAR_ALIVE),
            false => line_string.push_str(CHAR_DEAD)
        }
    }
    line_string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_line_returns_string() { 
        let test_cell_line = vec![grid::Cell::new(0, 0, true), grid::Cell::new(1, 0, false), grid::Cell::new(2, 0, true)];
        let result = generate_line(&test_cell_line);
        assert_eq!(result, "Ｏ　Ｏ")
    }
}