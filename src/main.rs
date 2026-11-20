
use std::env;
use std::time::Duration;
use std::thread;

mod board;
use board::{Board};

mod constants;
use constants::MILLI_GAP;

fn init_board(size: usize) -> Board {
	// Init board
	let mut board = Board::new(size);
	let middle = size / 2;

	// Init starting cells
	board.make_alive(middle, middle + 1);
    board.make_alive(middle + 1, middle + 2);
    board.make_alive(middle + 2, middle);
    board.make_alive(middle + 2, middle + 1);
    board.make_alive(middle + 2, middle + 2);

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
