use std::error::Error;
use std::fs::read_to_string;
use std::{fmt, ops};
use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let contents = read_to_string(filename)?;

    for line in contents.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

pub(crate) fn read_line(filename: &str) -> Result<String> {
    let result = read_lines(filename)?;
    if result.is_empty() {
        return Err(From::from(format!("No lines in file: {}", filename)));
    }
    if result.len() > 1 {
        return Err(From::from(format!("Expected only one line in file {}, but got: {}", filename, result.len())));
    }

    Ok(result.first().unwrap().clone())
}

pub(crate) fn parse_nums<T: FromStr>(content: &str) -> Vec<T>
    where <T as FromStr>::Err: Debug
{
    let pieces = content.split(' ')
        .map(|s| s.parse::<T>().expect(format!("Cannot parse {} as {}.", s, type_name::<T>()).as_str()))
        .collect_vec();
    pieces
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub const fn zero() -> Self { Self { x: 0, y: 0 } }
    pub const fn up() -> Self { Self { x: 0, y: -1 } }
    pub const fn down() -> Self { Self { x: 0, y: 1 } }
    pub const fn left() -> Self { Self { x: -1, y: 0 } }
    pub const fn right() -> Self { Self { x: 1, y: 0 } }

    pub fn rot_cw(&self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }
    pub fn rot_ccw(&self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: i32) -> Vec2 {
        Vec2::new(self.x * _rhs, self.y * _rhs)
    }
}

impl ops::Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self * _rhs.x, self * _rhs.y)
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

    pub(crate) fn get_int(&self, pos: Vec2) -> Option<i32> {
        if let Some(c) = self.get(pos) {
            return Some(c.to_digit(10).unwrap() as i32);
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
