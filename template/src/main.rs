// Global imports (needed for the simulation to run)
use crate::model::sea::Sea;
mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::sea_vis::SeaVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 100;

    let num_agents = 20;
    let dim: (f32, f32) = (400., 400.);

    let state = Sea::new(dim, num_agents);

    simulate!(state, step, 10);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.

    let num_agents = 10;
    let dim: (f32, f32) = (400., 400.);

    let state = Sea::new(dim, num_agents);
    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::BLUE)
        .with_name("Template")
        .start::<SeaVis, Sea>(SeaVis, state);
}
