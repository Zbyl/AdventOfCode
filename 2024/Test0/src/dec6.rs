use std::cmp::PartialEq;
use std::collections::HashSet;
use crate::helpers::{read_matrix, Matrix, Vec2};

#[derive(Debug)]
pub struct Maze {
    pub matrix: Matrix,
    pub start: Vec2,
}

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
    Up,
    Right,
    Down,
    Left,
}

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

fn walk_maze(matrix: &mut Matrix, start: Vec2, start_dir: Direction, mark: bool) -> bool {
    match matrix.get(start) {
        None => { panic!("Start position {start:?} not inside maze: {matrix:?}"); }
        Some(x) if x != '.' => { panic!("Start position {start:?} not on empty block, but on {x} inside maze: {matrix:?}"); }
        _ => {}
    }

    // Remember that we visited given position, going in the given direction.
    // If we'll get to that again, that means we're in a cycle.
    let mut visited = HashSet::<(Vec2, Direction)>::new();

    let mut pos = start;
    let mut cur_dir = start_dir;
    loop {
        if !matrix.contains(pos) {
            return true; // Escaped from the maze.
        }
        if mark {
            matrix.put(pos, 'X');
        }

        if visited.contains(&(pos, cur_dir)) {
            // We're in a cycle.
            return false;
        }
        visited.insert((pos, cur_dir));

        let save_dir = cur_dir;
        loop {
            let next_pos = pos + cur_dir.dir();
            let next_cell = matrix.get(next_pos).unwrap_or('.');
            if next_cell != '#' {
                pos = next_pos;
                break;
            }
            cur_dir = cur_dir.turn_cw();
            if cur_dir == save_dir {
                println!("Dude is trapped at: {:?}", pos);
                return false;
            }
        }
    }
}

fn try_trap(matrix: &mut Matrix, start: Vec2, start_dir: Direction) -> i32 {
    let mut result = 0;
    for x in 0 .. matrix.width {
        for y in 0 .. matrix.height {
            print!("{x} {y}");
            let pos = Vec2::new(x as i32, y as i32);
            if pos == start { println!(" start"); continue; }
            let c = matrix.get(pos).unwrap();
            if c != '.' { println!(" wall"); continue; }
            matrix.put(pos, '#');
            if walk_maze(matrix, start, start_dir, false) == false {
                result += 1;
                println!(" cycle");
            } else {
                println!(" ."); // escaped
            }
            matrix.put(pos, '.');
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec6() {
    let matrix = read_matrix("dec6.ex.txt").expect("Could not load input.");
    let mut maze = make_maze(matrix, '^');
    walk_maze(&mut maze.matrix, maze.start, Direction::Up, true);
    let result = maze.matrix.count('X');
    println!("{}", result);
    println!("{}", maze.matrix);
    println!("");
    println!("{}", result);
}

#[allow(dead_code)]
pub(crate) fn dec6_2() {
    let matrix = read_matrix("dec6.ex.txt").expect("Could not load input.");
    let mut maze = make_maze(matrix, '^');
    let result = try_trap(&mut maze.matrix, maze.start, Direction::Up);
    //println!("{}", result);
    //println!("{}", maze.matrix);
    //println!("");
    println!("{}", result);
}
