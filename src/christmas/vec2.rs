use std::ops::{Add, Sub};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0}
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
    pub fn between(start: Self, end: Self) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(start.x..end.x);
        let y = rng.gen_range(start.y..end.y);
        Self::new(x, y)
    }

    pub fn l2(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt() as f64
    }

    pub fn max(self) -> f64 {
        self.x.max(self.y)
    }
}