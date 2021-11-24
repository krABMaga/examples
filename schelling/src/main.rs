// Global imports (needed for the simulation to run)
use crate::model::world::Patch;
use crate::model::world::World;
mod model;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::*, rust_ab::simulate, rust_ab::Info, rust_ab::ProgressBar,
    std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::world_vis::WorldVis, rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem,
    rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D,
    rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D,
    rust_ab::visualization::visualization::Visualization,
};

pub const PERC: f32 = 0.5;
pub const PERCENT_SIMILAR_WANTED: f32 = 0.7;

/* pub static WIDTH: i32 = 100;
pub static HEIGHT: i32 = 100;
pub const NUM_AGENTS: u32 = 320; */

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 10;
    let dim: (i32, i32) = (100, 100);
    let num_agents = 320;

    let world = World::new(dim, num_agents);

    simulate!(step, world, 1, Info::Normal);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let dim: (i32, i32) = (100, 100);
    let num_agents = 320;
    let world = World::new(dim, num_agents);
    let mut app = Visualization::default()
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_window_dimensions(720., 720.)
        .with_background_color(Color::WHITE)
        .with_name("Schelling Model")
        .setup::<WorldVis, World>(WorldVis, world);
    app.add_system(SparseGrid2D::<Patch>::render.system());
    app.run();
}
