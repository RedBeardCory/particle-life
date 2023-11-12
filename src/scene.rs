use crate::{
    constants::{HEIGHT, WIDTH},
    creatures::Creature,
};
use log::{debug, log_enabled, Level};

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
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
            Self::check_collisions_and_update(creature);
        }
    }

    fn check_collisions_and_update(creature: &mut Creature) {
        // check scene collisions
        Self::check_scene_collisions(creature);
        // check collisions beteen creatures
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
