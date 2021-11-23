// Global imports (needed for the simulation to run)
use crate::model::sea::Sea;
mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::*, rust_ab::engine::state::State, rust_ab::simulate, rust_ab::Info,
    rust_ab::ProgressBar, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::sea_vis::SeaVis, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub const NUM_AGENTS: u32 = 10;
pub static WIDTH: f32 = 400.;
pub static HEIGHT: f32 = 400.;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    pub static STEP: u64 = 50;
    simulate!(STEP, Sea::new(), 1, Info::VERBOSE);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let state = Sea::new();
    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(500., 500.)
        .with_background_color(Color::BLUE)
        .with_name("Template")
        .start::<SeaVis, Sea>(SeaVis, state);
}
