use crate::model::state::Flocker;

mod model;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::simulate,
    rust_ab::Info, rust_ab::ProgressBar, std::time::Duration
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static WIDTH: f32 = 200.;
pub static HEIGHT: f32 = 200.;
pub static NUM_AGENTS: u32 = 100;

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    pub static STEP: u64 = 10;
    let state = Flocker::new();
    simulate!(STEP, state, 1, Info::VERBOSE);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = Flocker::new();
    Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_background_color(Color::rgb(0., 0., 0.))
        .with_name("Flockers")
        .start::<VisState, Flocker>(VisState, state);
}
