
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Board {
    size: u8,
    board :Vec<Vec<Cell>>,
}

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

impl Board {
    fn new(size: u8) -> Board {
        Board{
            size: size,
            board: vec![vec![Cell::Dead; size]; size],
        }
    }

    fn next(&mut self) {
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
                    new_board[y][x] = Cell::Alive;
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
}
