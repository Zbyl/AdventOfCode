use std::error::Error;
use std::{fmt, ops};
use std::fs::read_to_string;
use crate::dec4;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let contents = read_to_string(filename)?;

    for line in contents.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

#[derive(Debug)]
pub(crate) struct Matrix {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<String>,
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Matrix {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Matrix ({} x {}):", self.width, self.height)?;
        for row in self.data.iter() {
            writeln!(f, "  {}", row)?;
        }
        Ok(())
    }
}

impl Matrix {
    pub(crate) fn get(&self, pos: Vec2) -> Option<char> {
        if !self.contains(pos) {
            return None;
        }
        Some(self.data[pos.y as usize].as_bytes()[pos.x as usize].into())
    }

    pub(crate) fn put(&mut self, pos: Vec2, c: char) {
        if !self.contains(pos) {
            panic!("{:?} is out of bounds of {}", pos, self);
        }
        self.data[pos.y as usize].replace_range((pos.x as usize) .. ((pos.x + 1) as usize), &c.to_string());
    }

    pub(crate) fn contains(&self, pos: Vec2) -> bool {
        if pos.x < 0 || pos.y < 0 { return false; }
        if pos.x >= (self.width as i32) || pos.y >= (self.height as i32) { return false; }
        true
    }

    pub(crate) fn count(&self, c: char) -> i32 {
        let mut count = 0;
        for row in self.data.iter() {
            count += row.chars().filter(|ch| *ch == c).count() as i32;
        }
        count
    }
}

pub(crate) fn read_matrix(filename: &str) -> Result<Matrix> {
    let lines = read_lines(filename)?;
    if lines.is_empty() {
        return Ok(Matrix { width: 0, height: 0, data: vec![] })
    }

    let line_length = lines[0].len();
    for (idx, line) in lines.iter().skip(1).enumerate() {
        if line.len() != line_length {
            return Err(format!("Line idx={} (zero-based) {} has length {} should have {}.", idx, line, line.len(), line_length).into());
        }
    }

    Ok(Matrix { width: line_length, height: lines.len(), data: lines })
}

fn match_word(matrix: &Matrix, word: &str, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    for (idx, wc) in word.chars().enumerate() {
        let mc = matrix.get(Vec2::new(x + (idx as i32) * dx, y + (idx as i32) * dy)).unwrap_or('?');
        if mc != wc {
            return false
        }
    }
    true
}

fn match_dir(matrix: &Matrix, word: &str, dx: i32, dy: i32) -> i32 {
    let mut res = 0;
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            if match_word(matrix, word, x as i32, y as i32, dx, dy) {
                res += 1;
            }
        }
    }
    res
}

fn match_matrix(matrix: &Matrix, word: &str) -> i32 {
    0
        + match_dir(matrix, word, 1, 0)
        + match_dir(matrix, word, -1, 0)
        + match_dir(matrix, word, 0, 1)
        + match_dir(matrix, word, 0, -1)
        + match_dir(matrix, word, 1, 1)
        + match_dir(matrix, word, -1, -1)
        + match_dir(matrix, word, 1, -1)
        + match_dir(matrix, word, -1, 1)
}

fn match_x(matrix: &Matrix, x: i32, y: i32) -> bool {
    let diag_tl = match_word(matrix, "MAS", x - 1, y - 1, 1, 1);
    let diag_br = match_word(matrix, "MAS", x + 1, y + 1, -1, -1);
    let diag_tr = match_word(matrix, "MAS", x + 1, y - 1, -1, 1);
    let diag_bl = match_word(matrix, "MAS", x - 1, y + 1, 1, -1);
    (diag_tl || diag_br) && (diag_tr || diag_bl)
}

fn match_matrix_2(matrix: &Matrix) -> i32 {
    let mut res = 0;
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            if match_x(matrix, x as i32, y as i32) {
                res += 1;
            }
        }
    }
    res
}

#[allow(dead_code)]
pub(crate) fn dec4() {
    let matrix = dec4::read_matrix("dec4.in.txt").expect("Could not load input.");
    //println!("{:?}", matrix);
    let res = match_matrix(&matrix, "XMAS");
    println!("{:?}", res);
}

#[allow(dead_code)]
pub(crate) fn dec4_2() {
    let matrix = dec4::read_matrix("dec4.in.txt").expect("Could not load input.");
    //println!("{:?}", matrix);
    let res = match_matrix_2(&matrix);
    println!("{:?}", res);
}
