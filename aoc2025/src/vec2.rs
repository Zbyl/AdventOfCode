use std::ops;
use num::Signed;

#[allow(dead_code)]
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
