use aoc_lib::get_input_year;
use std::ops::{Add, AddAssign};

pub fn get_input(day: usize) -> String {
    get_input_year(2020, day)
}

impl Vec2d {
    pub fn left(&self) -> Vec2d {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn left_n(&self, n: usize) -> Vec2d {
        Self {
            x: self.x - n as isize,
            y: self.y,
        }
    }

    pub fn right(&self) -> Vec2d {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn right_n(&self, n: usize) -> Vec2d {
        Self {
            x: self.x + n as isize,
            y: self.y,
        }
    }

    pub fn up(&self) -> Vec2d {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn up_n(&self, n: usize) -> Vec2d {
        Self {
            x: self.x,
            y: self.y - n as isize,
        }
    }

    pub fn down(&self) -> Vec2d {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down_n(&self, n: usize) -> Vec2d {
        Self {
            x: self.x,
            y: self.y + n as isize,
        }
    }

    pub fn neighbours(&self) -> Vec<Vec2d> {
        vec![
            self.up().left(),
            self.up(),
            self.up().right(),
            self.right(),
            self.down().right(),
            self.down(),
            self.down().left(),
            self.left(),
        ]
    }

    pub fn manhattan_origin(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Vec2d {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Vec2d {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Vec2d {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
