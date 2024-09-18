use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    x: f32,
    y: f32,
}

impl Vec2d {

    pub fn new(x: f32, y: f32) -> Self {
        Vec2d { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn add(&mut self, other: &Vec2d) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn dot(&self, other: Vec2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn norm(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Sub<Vec2d> for Vec2d {

    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

}


impl Sub<f32> for Vec2d {

    type Output = Vec2d;

    fn sub(self, other: f32) -> Vec2d {
        Vec2d {
            x: self.x - other,
            y: self.y - other,
        }
    }

}


impl Add<Vec2d> for Vec2d {

    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}

impl Add<f32> for Vec2d {

    type Output = Vec2d;

    fn add(self, other: f32) -> Vec2d {
        Vec2d {
            x: self.x + other,
            y: self.y + other,
        }
    }

}

impl Div<f32> for Vec2d {

    type Output = Vec2d;

    fn div(self, other: f32) -> Vec2d {
        Vec2d {
            x: self.x / other,
            y: self.y / other,
        }
    }

}

impl Mul<f32> for Vec2d {

    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Vec2d {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

}

impl Mul<Vec2d> for f32 {
    type Output = Vec2d;

    fn mul(self, rhs: Vec2d) -> Vec2d {
        Vec2d {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}