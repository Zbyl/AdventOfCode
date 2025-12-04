use std::collections::{HashMap, HashSet};
use std::fmt;
use itertools::Itertools;
use crate::basic_parsing::read_lines;
use crate::vec2::Vec2;

#[allow(dead_code)]
#[derive(Debug, Clone)]
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

#[allow(dead_code)]
impl Matrix {
    pub fn new(width: usize, height: usize, fill: char, walls: Option<char>) -> Self {
        let mut matrix = Matrix {
            width: width,
            height: height,
            data: vec![fill.to_string().repeat(width); height],
        };

        if let Some(wall) = walls {
            for y in 0..height {
                for x in 0..width {
                    if (x == 0) || (x == width - 1) || (y == 0) || (y == height - 1) {
                        matrix.put(Vec2::new(x as i32, y as i32), wall);
                    }
                }
            }
        }

        matrix
    }

    pub(crate) fn get(&self, pos: Vec2) -> Option<char> {
        if !self.contains(pos) {
            return None;
        }
        Some(self.data[pos.y as usize].as_bytes()[pos.x as usize].into())
    }

    pub(crate) fn get_row(&self, row_idx: i32) -> Option<&str> {
        if !self.contains_row(row_idx) {
            return None;
        }
        Some(self.data[row_idx as usize].as_str())
    }

    pub(crate) fn get_int(&self, pos: Vec2) -> Option<i32> {
        if let Some(c) = self.get(pos) {
            return Some(c.to_digit(10).unwrap() as i32);
        }
        None
    }

    pub(crate) fn get_int_row(&self, row_idx: i32) -> Option<Vec<i32>> {
        if let Some(line) = self.get_row(row_idx) {
            return Some(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect_vec());
        }
        None
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

    pub(crate) fn contains_row(&self, row_idx: i32) -> bool {
        if row_idx < 0 { return false; }
        if row_idx >= (self.height as i32) { return false; }
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

#[allow(dead_code)]
pub(crate) fn read_matrix(filename: &str) -> crate::helpers::Result<Matrix> {
    let lines = read_lines(filename)?;
    read_matrix_from_lines(lines)
}

#[allow(dead_code)]
pub(crate) fn read_matrix_from_lines(lines: Vec<String>) -> crate::helpers::Result<Matrix> {
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

#[allow(dead_code)]
pub fn print_matrix(matrix: &Matrix, overrides: &HashMap<Vec2, char>) -> () {
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let val = if let Some(&c) = overrides.get(&pos) { c } else { matrix.get(pos).unwrap() };
            print!("{}", val);
        }
        println!();
    }
}

/// Collects locations of all points from 'marks'.
/// Optionally fills the found spots with provided 'fill' character.
#[allow(dead_code)]
pub fn find_points(matrix: &mut Matrix, fill: Option<char>, marks: &HashSet<char>) -> HashMap<char, Vec<Vec2>> {
    let mut result: HashMap<char, Vec<Vec2>> = HashMap::new();
    for (row_idx, row) in matrix.data.iter().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            if !marks.contains(&c) { continue; }
            let pos = Vec2::new(col_idx as i32, row_idx as i32);
            if !result.contains_key(&c) {
                result.insert(c, Vec::new());
            }
            result.get_mut(&c).unwrap().push(pos);
        }
    }

    if let Some(f) = fill {
        for (_c, positions) in result.iter() {
            for pos in positions {
                matrix.put(*pos, f);
            }
        }
    }
    result
}

/// Collects locations of all points from 'marks'.
/// Expects there to be at most one of each point type (exactly one if 'all' is true).
/// Optionally fills the found spots with provided 'fill' character.
#[allow(dead_code)]
pub fn find_single_points(matrix: &mut Matrix, fill: Option<char>, marks: &HashSet<char>, all: bool) -> HashMap<char, Vec2> {
    let pre_result = find_points(matrix, fill, marks);
    if all && (pre_result.len() != marks.len()) {
        panic!("Expected all {:?}, but found only: {:?}", marks, pre_result);
    }
    let mut result: HashMap<char, Vec2> = HashMap::new();
    for (c, positions) in pre_result {
        if positions.len() != 1 {
            panic!("Expected exactly one {:?}, but found {:?}", c, positions);
        }
        result.insert(c, positions[0]);
    }
    result
}
