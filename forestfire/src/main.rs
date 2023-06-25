// Global imports (needed for the simulation to run)
use crate::model::forest::Forest;
use crate::model::forest::Tree;

mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::simulate;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::forest_vis::ForestVis, krabmaga::bevy::prelude::Color,
    krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D,
    krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D,
    krabmaga::visualization::visualization::Visualization,
};

/* pub static STEP: u64 = 10;
pub static WIDTH: i32 = 6400;
pub static HEIGHT: i32 = 6400;
pub const DENSITY: f64 = 0.7; */

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 100;
    let dim: (i32, i32) = (200, 200);
    let density: f64 = 0.7;
    let forest = Forest::new(dim, density);
    let _ = simulate!(forest, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let dim: (i32, i32) = (50, 50);
    let density: f64 = 0.7;

    let state = Forest::new(dim, density);
    let mut app = Visualization::default()
        .with_simulation_dimensions(state.dim.0 as f32, state.dim.1 as f32)
        .with_window_dimensions(1000., 720.)
        .with_background_color(Color::BLACK)
        .with_name("Forest Fire Model")
        .setup::<ForestVis, Forest>(ForestVis, state);
    app.add_system(DenseGrid2D::<Tree>::render);
    app.run();
}
