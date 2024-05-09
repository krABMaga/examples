#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::map_vis::MapVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

// Global imports (needed for the simulation to run)
use crate::model::map::Map;

mod model;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.

    let num_agents = 0;
    let dim: (f32, f32) = (50., 50.);

    let state = Map::new(dim, num_agents);
    Visualization::default()
        .with_window_dimensions(400., 400.)
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_background_color(Color::NONE)
        .with_name("Gis")
        .start::<MapVis, Map>(MapVis, state);
}
