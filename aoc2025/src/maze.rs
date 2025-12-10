use crate::matrix::{print_matrix, Matrix};
use crate::vec2::Vec2;

#[derive(Debug)]
pub struct Maze {
    pub matrix: Matrix,
    pub start: Vec2,
}

#[allow(dead_code)]
pub fn make_maze(mut matrix: Matrix, start_char: char) -> Maze {
    for (row_idx, row) in matrix.data.iter_mut().enumerate() {
        if let Some(pos) = row.find(start_char) {
            let start = Vec2::new(pos as i32, row_idx as i32);
            matrix.put(start, '.');
            return Maze { matrix: matrix, start: start };
        }
    }
    panic!("No start position found in maze: {matrix:?}");
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    Up,         // Towards smaller y.
    Right,      // Towards larger x.
    Down,       // Towards larger y.
    Left,       // Towards smaller x.
}

#[allow(dead_code)]
impl Direction {
    pub fn turn_cw(&self) -> Self {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn turn_ccw(&self) -> Self {
        use Direction::*;
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn dir(&self) -> Vec2 {
        use Direction::*;
        match *self {
            Up => return Vec2::new(0, -1),
            Right => return Vec2::new(1, 0),
            Down => return Vec2::new(0, 1),
            Left => return Vec2::new(-1, 0),
        }
    }
}

#[allow(dead_code)]
pub fn print_maze(maze: &Maze, start_char: char) -> () {
    print_matrix(&maze.matrix, &hashmap! { maze.start => start_char })
}
