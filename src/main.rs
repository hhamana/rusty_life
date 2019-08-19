mod grid;
mod draw;
mod seeds;

fn main() {
    println!("Ctrl + C to stop the game.");
    let seed = seeds::random_seed();
    
    let grid = grid::Grid::new_from_seed(seed);
    // let mut iterations = 0;
    draw::to_sysout(&grid);
    print!("\x1B[2J");
    let new_grid = grid.tick();
    draw::to_sysout(&new_grid);
    // while iterations < 10 {
    //     iterations += 1;
    //     let grid = &grid.tick();
    // }
    
}
