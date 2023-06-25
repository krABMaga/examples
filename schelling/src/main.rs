// Global imports (needed for the simulation to run)
use crate::model::world::Patch;
use crate::model::world::World;
mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::world_vis::WorldVis, krabmaga::bevy::prelude::Color,
    krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D,
    krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D,
    krabmaga::visualization::visualization::Visualization,
};

pub const PERC: f32 = 0.5;
pub const SIMILAR_WANTED: u32 = 3;

/* pub static WIDTH: i32 = 100;
pub static HEIGHT: i32 = 100;
pub const NUM_AGENTS: u32 = 320; */

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    //testing deploy
    let step = 10;
    let dim: (i32, i32) = (20, 20);
    let num_agents = 320;

    let world = World::new(dim, num_agents);

    simulate!(world, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let dim: (i32, i32) = (25, 25);
    let num_agents = 320;
    let world = World::new(dim, num_agents);
    let mut app = Visualization::default()
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_window_dimensions(1000., 720.)
        .with_background_color(Color::WHITE)
        .with_name("Schelling Model")
        .setup::<WorldVis, World>(WorldVis, world);
    app.add_system(SparseGrid2D::<Patch>::render);
    app.run();
}
