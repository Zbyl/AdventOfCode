use std::cmp::PartialEq;
use crate::dec4;
use crate::dec4::{Matrix, Vec2};

#[derive(Debug)]
pub(crate) struct Maze {
    matrix: Matrix,
    start: Vec2,
}

fn make_maze(mut matrix: Matrix) -> Maze {
    for (row_idx, row) in matrix.data.iter_mut().enumerate() {
        if let Some(pos) = row.find('^') {
            let start = Vec2::new(pos as i32, row_idx as i32);
            matrix.put(start, '.');
            return Maze { matrix: matrix, start: start };
        }
    }
    panic!("No start position found in maze: {matrix:?}");
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn dir(&self) -> Vec2 {
        use Direction::*;
        match *self {
            Up => return Vec2::new(0, -1),
            Right => return Vec2::new(1, 0),
            Down => return Vec2::new(0, 1),
            Left => return Vec2::new(-1, 0),
        }
    }
}

fn walk_maze(matrix: &mut Matrix, start: Vec2, start_dir: Direction) -> () {
    match matrix.get(start) {
        None => { panic!("Start position {start:?} not inside maze: {matrix:?}"); }
        Some(x) if x != '.' => { panic!("Start position {start:?} not on empty block, but on {x} inside maze: {matrix:?}"); }
        _ => {}
    }

    let mut pos = start;
    let mut cur_dir = start_dir;
    loop {
        if matrix.count('.') == 0 {
            panic!("Matrix full, escape impossible: {matrix:?}");
        }
        if !matrix.contains(pos) {
            return;
        }
        matrix.put(pos, 'X');
        let save_dir = cur_dir;
        loop {
            let next_pos = pos + cur_dir.dir();
            let next_cell = matrix.get(next_pos).unwrap_or('.');
            if next_cell != '#' {
                pos = next_pos;
                break;
            }
            cur_dir = cur_dir.turn();
            if cur_dir == save_dir {
                panic!("Dude is trapped at: {:?}", pos);
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) fn dec6() {
    let matrix = dec4::read_matrix("dec6.in.txt").expect("Could not load input.");
    let mut maze = make_maze(matrix);
    walk_maze(&mut maze.matrix, maze.start, Direction::Up);
    let result = maze.matrix.count('X');
    println!("{}", result);
    println!("{}", maze.matrix);
    println!("");
    println!("{}", result);
}
