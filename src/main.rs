mod grid;
mod draw;
mod seeds;
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

const DELAY : u64 = 23;

fn main() {
    let now = Instant::now();

    // Here we go
    let iterations = 500;
	let (tx, rx) = mpsc::channel();
    // for i in 0..iterations {
    let txclone = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let seed = seeds::random_seed();
        let mut old_grid = grid::Grid::new_from_seed(seed);
        for _i in 0..iterations {
            let next_grid = old_grid.tick();
            old_grid = next_grid.tick();
            txclone.send(next_grid).unwrap();
            thread::sleep(Duration::from_millis(DELAY));
        }
    });

    for received in rx {
        print!("\x1B[2J");
		draw::to_sysout(received);
        println!("{:?}", now.elapsed());
    }

    // while iterations < 10 {
    //     iterations += 1;
    //     let grid = &grid.tick();
    // }
    
}
