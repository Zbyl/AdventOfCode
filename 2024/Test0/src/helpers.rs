use std::error::Error;
use std::fs::read_to_string;
use std::{fmt, ops};
use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;
use num::Signed;


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
pub(crate) struct Vec2<T = i32> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Signed + Copy> Vec2<T> {
    pub fn rot_cw(&self) -> Self {
        Self::new(-self.y, self.x)
    }
    pub fn rot_ccw(&self) -> Self {
        Self::new(self.y, -self.x)
    }
}

/*
const fn literal_hacky<T: Pod>(val: i8) -> T
{
    let size = std::mem::size_of::<T>();
    if size == std::mem::size_of::<i8>() { return bytemuck::cast(val) };
    if size == std::mem::size_of::<i32>() { return bytemuck::cast(val as i32) };
    if size == std::mem::size_of::<i64>() { return bytemuck::cast(val as i64) };
    unimplemented!();
}
*/

#[allow(dead_code)]
impl<T: Signed> Vec2<T> {
    pub fn zero() -> Self { Self { x: T::zero(), y: T::zero() } }
    pub fn up() -> Self { Self { x: T::zero(), y: -T::one() } }
    pub fn down() -> Self { Self { x: T::zero(), y: T::one() } }
    pub fn left() -> Self { Self { x: -T::one(), y: T::zero() } }
    pub fn right() -> Self { Self { x: T::one(), y: T::zero() } }
}

impl<T: Signed> ops::Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Signed> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl<T: Signed> ops::Sub<Self> for Vec2<T> {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl<T: Signed + Copy> ops::Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, _rhs: T) -> Self {
        Self::new(self.x * _rhs, self.y * _rhs)
    }
}

impl<T: Signed + Copy> ops::Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, _rhs: T) -> Self {
        Self::new(self.x / _rhs, self.y / _rhs)
    }
}
/*
impl<T> ops::Mul<Vec2<T>> for T
where
    T: ops::Mul<T, Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, _rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T>::new(self * _rhs.x, self * _rhs.y)
    }
}
*/
impl<T: Signed + ops::AddAssign> ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Signed + ops::SubAssign> ops::SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Signed + Copy + ops::MulAssign> ops::MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Signed + Copy + ops::DivAssign> ops::DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
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
