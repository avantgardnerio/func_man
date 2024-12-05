use std::ops::{Add, Div, Mul, Rem};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2d<T>(pub T, pub T)
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized;

impl<T> Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    pub fn len_sq(&self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T> From<[T; 2]> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    fn from(value: [T; 2]) -> Self {
        Self(value[0], value[1])
    }
}

impl<T> Rem<T> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn rem(self, rhs: T) -> Self::Output {
        [self.0 % rhs, self.1 % rhs].into()
    }
}

impl<T> Add<Vec2d<T>> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn add(self, rhs: Vec2d<T>) -> Self::Output {
        [self.0 + rhs.0, self.1 + rhs.1].into()
    }
}

impl<T> Div<T> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn div(self, rhs: T) -> Self::Output {
        [self.0 / rhs, self.1 / rhs].into()
    }
}

impl<T> Mul<T> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Eq + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn mul(self, rhs: T) -> Self::Output {
        [self.0 * rhs, self.1 * rhs].into()
    }
}
