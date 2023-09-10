// Global imports, required in all cases
use crate::model::state::ModelState;

pub mod model;

// Constants
pub const WIDTH: i32 = 200;
pub const HEIGHT: i32 = 200;
pub const NUM_AGENT: u32 = 100;
pub const EVAPORATION: f32 = 0.999;
pub const STEP: u64 = 1000;
// Nest coordinate range
pub const HOME_XMIN: i32 = 175;
pub const HOME_XMAX: i32 = 175;
pub const HOME_YMIN: i32 = 175;
pub const HOME_YMAX: i32 = 175;
// Food coordinate range
pub const FOOD_XMIN: i32 = 25;
pub const FOOD_XMAX: i32 = 25;
pub const FOOD_YMIN: i32 = 25;
pub const FOOD_YMAX: i32 = 25;
// Pheromone value
pub const HOME_LOW_PHEROMONE: f32 = 0.00000000000001;
pub const FOOD_LOW_PHEROMONE: f32 = 0.00000000000001;
// Ants action parameters
pub const REWARD: f32 = 1.;
pub const MOMENTUM_PROBABILITY: f64 = 0.8;
pub const RANDOM_ACTION_PROBABILITY: f64 = 0.1;
pub const UPDATE_CUTDOWN: f32 = 0.9;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::model::to_food_grid::ToFoodGrid, crate::model::to_home_grid::ToHomeGrid,
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::fields::number_grid_2d::BatchRender,
    krabmaga::visualization::visualization::Visualization,
};

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::simulate;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
pub mod visualization;

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = ModelState::new();
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(1280., 720.)
        .with_name("Ants foraging")
        .setup::<VisState, ModelState>(VisState, state);
    app.add_system(ToHomeGrid::batch_render)
        .add_system(ToFoodGrid::batch_render);
    app.run()
}

// #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
// use {krabmaga::rand, krabmaga::rand::Rng};

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let state = ModelState::new();

    let _ = simulate!(state, STEP, 10);
}
