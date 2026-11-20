
use std::env;
use std::time::Duration;
use std::thread;
use rand::RngExt;

mod board;
use board::{Board};

mod constants;
use constants::MILLI_GAP;

fn init_board(size: usize) -> Board {
    let mut board = Board::new(size);
	let mut rng = rand::rng();

	// Make alive random cells
	for y in 0..size {
		for x in 0..size {
			if rng.random_bool(0.35) {
				board.make_alive(x, y);
			}
		}
	}

    board
}

fn main() {
	// Get board size
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		return
	}
	let size: usize = args[1].parse().expect("invalid size");

	use init_board;

	// Init board
    let mut board = init_board(size);

	// Main loop
	loop {
		// Delete previous print
		print!("\x1B[2J\x1B[1;1H");

		board.display();
		board.next();

		// Keep a gap between steps
		thread::sleep(Duration::from_millis(MILLI_GAP));
	}
}
