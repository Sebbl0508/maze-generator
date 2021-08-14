use std::ops::{Add, AddAssign};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Add<Vec2<usize>> for Vec2<isize> {
    type Output = Vec2<isize>;

    fn add(self, rhs: Vec2<usize>) -> Self::Output {
        Vec2::new(
            self.x + rhs.x as isize,
            self.y + rhs.y as isize
        )
    }
}

impl Add<Vec2<isize>> for Vec2<usize> {
    type Output = Vec2<usize>;

    fn add(self, rhs: Vec2<isize>) -> Self::Output {
        Vec2::new(
            ((self.x as isize) + rhs.x) as usize,
            ((self.y as isize) + rhs.y) as usize
        )
    }
}

impl<T> Add for Vec2<T>
where T: Add<Output = T> + Copy {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}





// Hmhmhmh, idk, just do this :/
/*
impl Add<isize> for usize {
    type Output = usize;

    fn add(self, rhs: isize) -> Self::Output {
        return if rhs < 0 {
            self - (rhs * 1)
        } else {
            self + rhs
        }
    }
}
 */