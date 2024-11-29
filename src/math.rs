use std::ops::{Add, Div, Mul, Rem};

#[derive(Debug, Copy, Clone)]
pub struct Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    pub val: [T; 2],
}

impl<T> Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    pub fn len_sq(&self) -> T {
        self.val[0] * self.val[0] + self.val[1] * self.val[1]
    }
}

impl<T> From<[T; 2]> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    fn from(value: [T; 2]) -> Self {
        Self { val: value }
    }
}

impl<T> Rem<T> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn rem(self, rhs: T) -> Self::Output {
        [self.val[0] % rhs, self.val[1] % rhs].into()
    }
}

impl<T> Add<Vec2d<T>> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn add(self, rhs: Vec2d<T>) -> Self::Output {
        [self.val[0] + rhs.val[0], self.val[1] + rhs.val[1]].into()
    }
}

impl<T> Div<T> for Vec2d<T>
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Sized,
{
    type Output = Vec2d<T>;

    fn div(self, rhs: T) -> Self::Output {
        [self.val[0] / rhs, self.val[1] / rhs].into()
    }
}
