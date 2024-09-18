use macroquad::prelude::{draw_circle, next_frame, clear_background, WHITE, BLACK};
use macroquad;
use rand::{self, Rng};
use rand::thread_rng;
use trail::Trail;

mod vec2d;
mod trail;

struct Body {
    position: vec2d::Vec2d,
    velocity: vec2d::Vec2d,
    mass: f32,
    radius: f32,
    color: macroquad::color::Color,
    trail: Trail
}

impl Body {

    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(20..40) as f32;
        let position = vec2d::Vec2d::new(rng.gen_range(0..macroquad::window::screen_width() as i32) as f32 + radius, rng.gen_range(0..macroquad::window::screen_height() as i32) as f32 + radius );

        let velocity = vec2d::Vec2d::new(rng.gen_range(0..30) as f32, rng.gen_range(0..30) as f32);
        let mass = rng.gen_range(1..10) as f32;
        let color = macroquad::color::WHITE;
        let trail = Trail::new(100);
        Body {
            position,
            velocity,
            mass,
            radius,
            color,
            trail
        }
    }

    fn position(&self) -> vec2d::Vec2d {
        self.position.clone()
    }

    fn velocity(&self) -> vec2d::Vec2d {
        self.velocity.clone()
    }

    fn mass(&self) -> f32 {
        self.mass.clone()
    }

    fn draw(&self) {
        draw_circle(self.position.x(),
            self.position.y(),
            self.radius,
            self.color);

        self.trail.draw();
    }

    fn move_body(&mut self, dt: f32) {
        self.position.set_x(self.position.x() + self.velocity.x() * dt);
        self.position.set_y(self.position.y() + self.velocity.y() * dt);

        self.trail.update(self.position);
    }

    fn check_boundary_collision(&mut self) {

        if self.position.x() + self.radius >= macroquad::window::screen_width()
            || self.position.x() - self.radius <= 0.0 {
                self.velocity.set_x(self.velocity.x() * -1.0);
            }

        if self.position.y() + self.radius >= macroquad::window::screen_height()
            || self.position.y() - self.radius <= 0.0 {
                self.velocity.set_y(self.velocity.y() * -1.0);
            }
    }

    fn distance(&self, other: &Body) -> f32 {
        ((self.position.x() - other.position.x()).powf(2.0) + (self.position.y() - other.position.y()).powf(2.0)).sqrt()
    }

    fn handle_collision(&mut self, other: &mut Body) {
        if self.radius + other.radius >= self.distance(other) {
            let vA = self.velocity();
            let vB = other.velocity();
            let rA = self.position();
            let rB = other.position();
            let M = self.mass + other.mass;
            let d : vec2d::Vec2d = rA - rB;
            let new_velocity_a = vA - 2.0 * other.mass / M * (vA - vB).dot(rA - rB) * d / d.norm();
            let new_velocity_b = vB + 2.0 * self.mass / M * (vB - vA).dot(rB - rA) * d / d.norm();
            self.set_velocity(new_velocity_a);
            other.set_velocity(new_velocity_b);
        }

    }

    fn set_velocity(&mut self, velocity: vec2d::Vec2d) {
        self.velocity = velocity;
    }

}

#[macroquad::main("MyGame")]
async fn main() {
    let dt = 0.1;
    let mut b1 = Body::new();
    let mut b2 = Body::new();
    let mut bodies = [b1, b2];
    loop {
        clear_background(BLACK);

        for i in 0..bodies.len() {
            let b = &mut bodies[i];
            b.draw();
            b.move_body(dt);
            b.check_boundary_collision();
            for j in i + 1..bodies.len() {
                let (left, right) = bodies.split_at_mut(j);
                left[i].handle_collision(&mut right[0]);
            }

        }

        // Check collisions between bodies

        next_frame().await
    }
}