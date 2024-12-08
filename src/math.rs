use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Vec2dOps:
    Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Into<f64>
    + Eq
    + Copy
    + Sized
{
}

impl<T> Vec2dOps for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + Into<f64>
        + Eq
        + Copy
        + Sized
{
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2d<T>
where
    T: Vec2dOps,
{
    pub x: T,
    pub y: T,
}

impl<T> Vec2d<T>
where
    T: Vec2dOps,
{
    pub fn len_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f64 {
        self.len_sq().into().sqrt()
    }
}

impl<T> From<[T; 2]> for Vec2d<T>
where
    T: Vec2dOps,
{
    fn from(value: [T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T> Rem<T> for Vec2d<T>
where
    T: Vec2dOps,
{
    type Output = Vec2d<T>;

    fn rem(self, rhs: T) -> Self::Output {
        [self.x % rhs, self.y % rhs].into()
    }
}

impl<T> Add<Vec2d<T>> for Vec2d<T>
where
    T: Vec2dOps,
{
    type Output = Vec2d<T>;

    fn add(self, rhs: Vec2d<T>) -> Self::Output {
        [self.x + rhs.x, self.y + rhs.y].into()
    }
}

impl<T> Sub<Vec2d<T>> for Vec2d<T>
where
    T: Vec2dOps,
{
    type Output = Vec2d<T>;

    fn sub(self, rhs: Vec2d<T>) -> Self::Output {
        [self.x - rhs.x, self.y - rhs.y].into()
    }
}

impl<T> Div<T> for Vec2d<T>
where
    T: Vec2dOps,
{
    type Output = Vec2d<T>;

    fn div(self, rhs: T) -> Self::Output {
        [self.x / rhs, self.y / rhs].into()
    }
}

impl<T> Mul<T> for Vec2d<T>
where
    T: Vec2dOps,
{
    type Output = Vec2d<T>;

    fn mul(self, rhs: T) -> Self::Output {
        [self.x * rhs, self.y * rhs].into()
    }
}
