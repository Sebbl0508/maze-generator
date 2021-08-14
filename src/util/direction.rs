use crate::util::vec2::Vec2;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn vector(&self) -> Vec2<isize> {
        return match *self {
            Direction::Up => {
                Vec2::new(0, 1)
            }
            Direction::Down => {
                Vec2::new(0, -1)
            }
            Direction::Left => {
                Vec2::new(-1, 0)
            }
            Direction::Right => {
                Vec2::new(1, 0)
            }
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
    }

    pub fn random() -> Direction {
        use rand::prelude::*;
        let mut rng = thread_rng();
        Self::all().choose(&mut rng).unwrap().clone()
    }
}