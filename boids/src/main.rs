// Global imports
use crate::model::bird::Bird;
use crate::model::boids_state::{BoidsState, DISCRETIZATION, HEIGHT, TOROIDAL, WIDTH};
use rust_ab::engine::schedule::Schedule;

mod model;

// Constants
static NUM_AGENT: u128 = 1000;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState,
    rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = BoidsState::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);
    let schedule: Schedule<Bird> = Schedule::new();

    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_background_color(Color::rgb(0., 0., 0.))
        .start::<Bird, VisState>(VisState, state, schedule);
}

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::rand,
    rust_ab::rand::Rng,
    rust_ab::engine::location::Real2D,
    rust_ab::simulate
};

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    static STEP: u128 = 50;

    let mut rng = rand::thread_rng();
    let mut schedule: Schedule<Bird> = Schedule::new();
    // assert!(schedule.events.is_empty());

    let mut state = BoidsState::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);
    for bird_id in 0..NUM_AGENT {

        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();
        let last_d = Real2D { x: 0., y: 0. };
        let bird = Bird::new(
            bird_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGHT * r2,
            },
            last_d,
        );
        state
            .field1
            .set_object_location(bird, bird.pos);

        schedule.schedule_repeating(bird, 0., 0);
    }

    // assert!(!schedule.events.is_empty());

    
    simulate!(STEP, schedule, Bird, state);
}