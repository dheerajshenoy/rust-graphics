use macroquad::{color::WHITE, shapes::draw_line};

use crate::vec2d::{self, Vec2d};

pub struct Trail {
    positions: Vec<Vec2d>,
    max_length: usize,
}

impl Trail {

    pub fn new(max_length: usize) -> Self {
        Trail {
            positions: Vec::new(),
            max_length,
        }
    }

    pub fn update(&mut self, pos: Vec2d) {
        self.positions.push(pos);
        if self.positions.len() > self.max_length {
            self.positions.remove(0);
        }
    }

    pub fn draw(&self) {
        for i in 1..self.positions.len() {
            draw_line(
                self.positions[i - 1].x(),
                self.positions[i - 1].y(),
                self.positions[i].x(),
                self.positions[i].y(),
                2.0,
                WHITE);
        }
    }
}