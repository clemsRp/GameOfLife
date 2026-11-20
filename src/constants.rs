
pub const MILLI_GAP: u64 = 200;

pub const ALIVE_CHAR: &str = "#";
pub const DEAD_CHAR: &str = ".";

pub const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
