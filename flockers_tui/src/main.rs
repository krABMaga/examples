use crate::model::state::Flocker;

mod model;

use rust_ab::*;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State,
    rust_ab::Info, rust_ab::ProgressBar, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

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
    let step = 2;

    let dim = (200., 200.);
    let num_agents = 10;
  
    let state = Flocker::new(dim, num_agents);
    simulate!(state, step, 30);
}

// Main used when a visualization feature is applied.
// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
// fn main() {
//     let dim = (200., 200.);
//     let num_agents = 100;
//     let state = Flocker::new(dim, num_agents);
//     Visualization::default()
//         .with_window_dimensions(1000., 700.)
//         .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
//         .with_background_color(Color::rgb(0., 0., 0.))
//         .with_name("Flockers")
//         .start::<VisState, Flocker>(VisState, state);
// }
