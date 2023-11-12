use log::{debug, info, log_enabled, Level};
use macroquad::prelude::*;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const NUM_CREATURES: i32 = 10;
const CREATURE_SIZE: i32 = 10;
const CREATURE_VEL_MAX: f32 = 100.0;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Scene {
    creatures: Vec<Creature>,
}

impl Scene {
    fn new() -> Self {
        Scene {
            creatures: init_creatures(NUM_CREATURES),
        }
    }

    fn display(&self) {
        for creature in self.creatures.iter() {
            creature.display();
        }
    }

    fn update(&mut self, dt: f32) {
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

#[derive(Debug)]
struct Creature {
    position: Point,

    /// Measured in pixels per second
    velocity: Vec2,

    /// Radius to determine the size of the point
    radius: f32,

    /// Colour of circle
    color: Color,
}

impl Creature {
    fn new() -> Self {
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

    fn display(&self) {
        // could be cool to make colour change according to velocity
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }

    /// takes the change in time and get's the new position
    fn update(&mut self, dt: f32) {
        self.position = Point {
            x: self.position.x + self.velocity.x * dt,
            y: self.position.y + self.velocity.y * dt,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
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

#[macroquad::main(window_conf)]
async fn main() {
    // init logger
    env_logger::init();

    // Example of checking log level before logging intensive messages
    if log_enabled!(Level::Debug) {
        debug!("Logging initialised");
        debug!("screen_width: {}", screen_width());
        debug!("screen_height: {}", screen_height());
    }

    info!("Starting simulation");

    // need to request the window to update
    request_new_screen_size(WIDTH as f32, HEIGHT as f32);
    next_frame().await;

    let mut scene = Scene::new();

    if log_enabled!(Level::Debug) {
        debug!("{:#?}", scene.creatures);
    }

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 1.0, RED);

        scene.display();

        // update scene
        // get time since last frame for calculations
        let dt = get_frame_time();
        scene.update(dt);

        next_frame().await;
    }
}
