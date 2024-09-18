use std::any::{type_name, Any};
use std::time::{self, SystemTime, UNIX_EPOCH};
use std::vec;
use macroquad::color::Color;
use macroquad::input::{is_key_down, is_key_pressed};
use macroquad::prelude::{draw_circle, next_frame, clear_background, WHITE, BLACK};
use macroquad;
use macroquad::prelude::KeyCode;
use macroquad::text::{draw_text, get_text_center};
use macroquad::rand;
use macroquad::window::screen_height;
use trail::Trail;
use vec2d::Vec2d;

mod vec2d;
mod trail;

const G: f32 = 1.0;  // Gravitational constant (scaled for simulation)

struct Body {
    position: Vec2d,
    velocity: Vec2d,
    mass: f32,
    radius: f32,
    color: macroquad::color::Color,
    trail: Trail
}

impl Body {

    fn random() -> Self {
        let time = SystemTime::now();
        rand::srand(time.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as u64);
        let radius = rand::gen_range(1, 10) as f32;
        let x = rand::gen_range(0, macroquad::window::screen_width() as i32) as f32;
        let y = rand::gen_range(0, macroquad::window::screen_height() as i32) as f32;
        let position = Vec2d::new(x, y);
        let velocity = Vec2d::new(rand::gen_range(0, 3) as f32, rand::gen_range(0, 3) as f32);
        let mass = rand::gen_range(1, 10) as f32;
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

    fn new(position: Vec2d, velocity: Vec2d,
        mass: f32, radius: f32, color: Color) -> Self {
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

    fn position(&self) -> Vec2d {
        self.position.clone()
    }

    fn velocity(&self) -> Vec2d {
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
    }

    fn draw_trails(&self) {
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
            let d : Vec2d = rA - rB;
            let new_velocity_a = vA - 2.0 * other.mass / M * (vA - vB).dot(rA - rB) * d / d.norm();
            let new_velocity_b = vB + 2.0 * self.mass / M * (vB - vA).dot(rB - rA) * d / d.norm();
            self.set_velocity(new_velocity_a);
            other.set_velocity(new_velocity_b);
        }

    }

    fn set_velocity(&mut self, velocity: Vec2d) {
        self.velocity = velocity;
    }

    fn apply_gravity(&mut self, other: &mut Body) {
        // Calculate distance vector between the bodies
        let distance_vector = other.position - self.position;
        let distance = distance_vector.norm();

        // Avoid division by zero or too small distances
        if distance > 1.0 {
            // Calculate gravitational force magnitude
            let force_magnitude = G * (self.mass * other.mass) / distance.powf(2.0);

            // Normalize distance vector and scale it by force magnitude
            let force_vector = distance_vector.normalize() * force_magnitude;

            // Apply force as acceleration (F = ma, so a = F / m)
            let acceleration = force_vector / self.mass;

            // Update velocity by the acceleration
            self.set_velocity(self.velocity + acceleration);
        }
    }

    fn draw_info(&self) {
        draw_text(&format!("M: {}, V: {}", self.mass, self.velocity),
            self.position.x() + 20.0,
            self.position.y() + 5.0,
            24.0,
            WHITE
        );

    }


    fn overlaps(&self, other: &Body) -> bool {
        (self.position - other.position).norm() <= (self.radius + other.radius)
    }

}

#[macroquad::main("MyGame")]
async fn main() {
    let dt = 0.1;
    let mut collision_state = false;
    let mut boundary_collision_state = false;
    let mut body_info_state = false;
    let mut show_info_state = true;
    let mut trails_state = true;
    
    // let mut b1 = Body::new(Vec2d::new(400.0, 50.0),
    //     Vec2d::new(10.0, 5.0),
    //     0.5,
    //     4.0,
    //     WHITE);

    // let mut b2 = Body::new(Vec2d::new(300.0, 400.0),
    //     Vec2d::new(0.0, 0.0),
    //     10000.0,
    //     10.0,
    //     WHITE);

    let mut bodies = vec![];
    // let mut bodies = vec![b1, b2];

    loop {

        if is_key_pressed(KeyCode::Space) {
            bodies.clear();

            let mut b1 = Body::random();
            let mut b2 = Body::random();
            let mut b3 = Body::random();
            
            while b1.overlaps(&b2) {
                b2 = Body::random();
            }

            while b2.overlaps(&b3) || b1.overlaps(&b3) {
                b3 = Body::random();
            }

            bodies.push(b1);
            bodies.push(b2);
            bodies.push(b3);
        }

        if is_key_pressed(KeyCode::C) {
            collision_state = !collision_state;
        }

        if is_key_pressed(KeyCode::B) {
            boundary_collision_state = !boundary_collision_state;
        }

        if is_key_pressed(KeyCode::I) {
            body_info_state = !body_info_state;
        }

        if is_key_pressed(KeyCode::U) {
            show_info_state = !show_info_state;
        }

        if is_key_pressed(KeyCode::T) {
            trails_state = !trails_state;
        }

        clear_background(BLACK);

        for i in 0..bodies.len() {
            let b = &mut bodies[i];
            b.draw();

            b.move_body(dt);

            // If trails enabled
            if trails_state {
                b.draw_trails();
            }
                
            // If boundary collision enabled
            if boundary_collision_state {
                b.check_boundary_collision();
            }

            // If body info enabled
            if body_info_state {
                b.draw_info();
            }

            for j in i + 1..bodies.len() {
                let (left, right) = bodies.split_at_mut(j);

                // If particle collision enabled
                if collision_state {
                    left[i].handle_collision(&mut right[0]);
                }

                left[i].apply_gravity(&mut right[0]);
            }
        }

        if show_info_state {
            draw_text("Restart (R)",
                10.0, screen_height() - 190.0, 24.0, WHITE);

            draw_text("Randomize (Space)",
                10.0, screen_height() - 160.0, 24.0, WHITE);

            draw_text(&format!("Trails (T): [{}]", trails_state),
                10.0, screen_height() - 130.0, 24.0, WHITE);

            draw_text(&format!("Collision (C): [{}]", collision_state),
                10.0, screen_height() - 100.0, 24.0, WHITE);

            draw_text(&format!("Boundary Collision (B): [{}]", boundary_collision_state),
                10.0, screen_height() - 70.0, 24.0, WHITE);

            draw_text(&format!("Show Body Info (I): [{}]", body_info_state),
                10.0, screen_height() - 40.0, 24.0, WHITE);

            draw_text(&format!("Show This Info (U): [{}]", show_info_state),
                10.0, screen_height() - 10.0, 24.0, WHITE);
        }

        next_frame().await;
    }
 }