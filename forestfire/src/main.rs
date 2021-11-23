// Global imports (needed for the simulation to run)
use crate::model::forest::Forest;
use crate::model::forest::Tree;

mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::*, rust_ab::simulate, rust_ab::Info, rust_ab::ProgressBar,
    std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::forest_vis::ForestVis, rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem, rust_ab::engine::fields::dense_object_grid_2d::DenseGrid2D,
    rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D,
    rust_ab::visualization::visualization::Visualization,
};

pub static STEP: u64 = 10;
pub static WIDTH: i32 = 6400;
pub static HEIGHT: i32 = 6400;
pub const DENSITY: f64 = 0.7;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    simulate!(STEP, Forest::new(), 1, Info::VERBOSE);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let state = Forest::new();
    let mut app = Visualization::default()
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(720., 720.)
        .with_background_color(Color::BLACK)
        .with_name("Forest Fire Model")
        .setup::<ForestVis, Forest>(ForestVis, state);
    app.add_system(DenseGrid2D::<Tree>::render.system());
    app.run();
}
