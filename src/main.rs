mod grid;
mod draw;
mod seeds;
use std::thread;
use std::time::{Instant, Duration};
use std::sync::mpsc;

const DELAY : u64 = 23;

fn main() {
    let now = Instant::now();

    // Sender/Receiver pattern on 2 threads
    let (tx, rx) = mpsc::channel();

    // Sender thread generates ticks
    thread::spawn(move || {
        let seed = seeds::random_seed();
        let mut old_grid = grid::Grid::new_from_seed(seed);
        loop {
            let next_grid = old_grid.tick();
            old_grid = next_grid.tick();
            tx.send(next_grid).unwrap();
            thread::sleep(Duration::from_millis(DELAY));
        }
    });
    // Receiver (also main) thread draws
    for received in rx {
        print!("\x1B[2J");
		draw::to_sysout(received);
        println!("{:?} elapsed. Press Ctrl+C or close the terminal to kill the game.", now.elapsed());
    }
}
