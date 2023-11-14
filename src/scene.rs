use crate::{
    constants::{HEIGHT, WIDTH},
    creatures::Creature,
};
use log::{debug, log_enabled, Level};

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn distance_from(&self, point: &Point) -> f32 {
        let a_sqr = (point.x - self.x).powi(2);
        let b_sqr = (point.y - self.y).powi(2);
        (a_sqr + b_sqr).sqrt()
    }
}

#[derive(Debug)]
pub struct Scene {
    pub creatures: Vec<Creature>,
}

/// Initialise creature list with random creatures
fn init_creatures(num_creatures: i32) -> Vec<Creature> {
    let mut creatures: Vec<Creature> = Vec::new();
    for _ in 0..num_creatures {
        let new_creature = Creature::new();
        if log_enabled!(Level::Debug) {
            debug!("{:#?}", new_creature);
        }
        creatures.push(new_creature);
    }

    creatures
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            creatures: init_creatures(crate::constants::NUM_CREATURES),
        }
    }

    pub fn display(&self) {
        for creature in self.creatures.iter() {
            creature.display();
        }
    }

    pub fn update(&mut self, dt: f32) {
        // update all the creature's positions
        for creature in self.creatures.iter_mut() {
            creature.update(dt);
            Self::check_scene_collisions(creature);
        }

        // use buffer to work around borrowing issues
        let mut creature_buffer = self.creatures.clone();

        for (i, creature_a) in self.creatures.iter().enumerate() {
            for creature_b in self.creatures.iter() {
                // check if they are the same and skip
                if creature_a == creature_b {
                    continue;
                }

                // check if they collide
                // check the distance between the points
                let dist = creature_a
                    .position
                    .distance_from(&creature_b.position)
                    .abs();
                if dist <= creature_a.radius + creature_b.radius {
                    // collided
                    // change color as indication
                    creature_buffer.get_mut(i).unwrap().color =
                        crate::creatures::get_random_color();

                    // TODO: work out direction to move
                }
            }
        }

        // swap out buffer
        self.creatures = creature_buffer;
    }

    fn check_scene_collisions(creature: &mut Creature) {
        // check if the creature has hit the edges
        if creature.position.x + creature.radius > WIDTH as f32 {
            // reverse x
            if log_enabled!(Level::Debug) {
                debug!("Collision detected! position.x {}", creature.position.x);
            }
            creature.velocity.x = -creature.velocity.x;
            // pop the creature back out to just before the collision
            creature.position.x = WIDTH as f32 - creature.radius;
        }

        if creature.position.x - creature.radius < 0.0 {
            // reverse x
            if log_enabled!(Level::Debug) {
                debug!("Collision detected! position.x {}", creature.position.x);
            }
            creature.velocity.x = -creature.velocity.x;
            // pop the creature back out to just before the collision
            creature.position.x = creature.radius;
        }

        if creature.position.y + creature.radius > HEIGHT as f32 {
            // reverse y
            if log_enabled!(Level::Debug) {
                debug!("Collision detected! position.y {}", creature.position.y);
            }
            creature.velocity.y = -creature.velocity.y;
            creature.position.y = HEIGHT as f32 - creature.radius;
        }

        if creature.position.y - creature.radius < 0.0 {
            // reverse y
            if log_enabled!(Level::Debug) {
                debug!("Collision detected! position.y {}", creature.position.y);
            }
            creature.velocity.y = -creature.velocity.y;
            creature.position.y = creature.radius;
        }
    }
}
