
use crate::constants::{OFFSETS, ALIVE_CHAR, DEAD_CHAR};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

pub struct Board {
    size: usize,
    board :Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board{
            size: size,
            board: vec![vec![Cell::Dead; size]; size],
        }
    }

    pub fn next(&mut self) {
        // Init new board
        let mut new_board = Board::new(self.size);

        // Update new board cells
        for y in 0..self.size {
            for x in 0..self.size {
                // Setup living conditions
                let n = self.alive_neighbours(x, y);
                let born = self.board[y][x] == Cell::Dead && n == 3;
                let stay_alive = self.board[y][x] == Cell::Alive && (n == 2 || n == 3);

                // Update cell status if needed
                if born || stay_alive {
                    new_board.board[y][x] = Cell::Alive;
                }
            }
        }

        // Replace previous board with new one
        *self = new_board;
    }

    fn alive_neighbours(&self, x: usize, y: usize) -> usize {
        OFFSETS
            .iter()
            .filter(|(dx, dy)| {
                let cx = x as i32 + dx;
                let cy = y as i32 + dy;

                let inside = cx >= 0
                    && cy >= 0
                    && (cx as usize) < self.size
                    && (cy as usize) < self.size;

                inside && self.board[cy as usize][cx as usize] == Cell::Alive
            })
            .count()
    }

	pub fn make_alive(&mut self, x: usize, y: usize) {
		if x < self.size && y < self.size {
			self.board[y][x] = Cell::Alive;
		}
	}

	pub fn display(&self) {
		// Range y and x
		for y in 0..self.size {
			for x in 0..self.size {
				// Display character
				let char = {
					if self.board[y][x] == Cell::Alive {
						ALIVE_CHAR
					} else {
						DEAD_CHAR
					}
				};

				print!("{}", char);
			}

			print!("\n");
		}
	}
}
