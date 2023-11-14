use log::{debug, info, log_enabled, Level};
use macroquad::{
    prelude::{BLACK, RED},
    shapes::draw_rectangle_lines,
    time::get_frame_time,
    window::{
        clear_background, next_frame, request_new_screen_size, screen_height, screen_width, Conf,
    },
};
use particle_life::{
    constants::{HEIGHT, WIDTH},
    scene::Scene,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
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

        // draws border around play area
        draw_rectangle_lines(0.0, 0.0, WIDTH as f32, HEIGHT as f32, 1.0, RED);

        scene.display();

        // update scene
        // get time since last frame for calculations
        let dt = get_frame_time();
        scene.update(dt);

        next_frame().await;
    }
}
