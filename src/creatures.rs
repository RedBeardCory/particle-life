use macroquad::{
    prelude::{rand, Color, Vec2},
    shapes::draw_circle,
};

use crate::{
    constants::{CREATURE_SIZE, CREATURE_VEL_MAX, HEIGHT, WIDTH},
    scene::Point,
};

#[derive(Debug)]
pub struct Creature {
    pub position: Point,

    /// Measured in pixels per second
    pub velocity: Vec2,

    /// Radius to determine the size of the point
    pub radius: f32,

    /// Colour of circle
    color: Color,
}

impl Creature {
    pub fn new() -> Self {
        let radius = CREATURE_SIZE as f32;
        Creature::new_at_point(
            Point {
                x: rand::gen_range(0, HEIGHT) as f32,
                y: rand::gen_range(0, WIDTH) as f32,
            },
            Vec2::new(
                rand::gen_range(-CREATURE_VEL_MAX, CREATURE_VEL_MAX),
                rand::gen_range(-CREATURE_VEL_MAX, CREATURE_VEL_MAX),
            ),
            radius,
        )
    }

    fn new_at_point(position: Point, velocity: Vec2, radius: f32) -> Self {
        Creature {
            position,
            velocity,
            radius,
            color: Color::from_rgba(
                rand::gen_range(0, 255),
                rand::gen_range(0, 255),
                rand::gen_range(0, 255),
                255,
            ),
        }
    }

    pub fn display(&self) {
        // could be cool to make colour change according to velocity
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }

    /// takes the change in time and get's the new position
    pub fn update(&mut self, dt: f32) {
        self.position = Point {
            x: self.position.x + self.velocity.x * dt,
            y: self.position.y + self.velocity.y * dt,
        }
    }
}
