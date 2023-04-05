use crate::model::state::WsgState;
mod model;

pub const ENERGY_CONSUME: f64 = 1.0;

pub const FULL_GROWN: u16 = 20;

pub const GAIN_ENERGY_SHEEP: f64 = 4.0;
pub const GAIN_ENERGY_WOLF: f64 = 20.0;

pub const SHEEP_REPR: f64 = 0.2;
pub const WOLF_REPR: f64 = 0.1;

pub const MOMENTUM_PROBABILITY: f64 = 0.8;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 200;

    let dim: (i32, i32) = (50, 50);
    let initial_animals: (u32, u32) = ((200. * 0.6) as u32, (200. * 0.4) as u32);

    let state = WsgState::new(dim, initial_animals);
    let _ = simulate!(state, step, 10);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    krabmaga::visualization::fields::number_grid_2d::BatchRender,
    krabmaga::visualization::visualization::Visualization,
};

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let dim: (i32, i32) = (25, 25);

    let initial_animals: (u32, u32) = ((60. * 0.6) as u32, (60. * 0.4) as u32);

    let state = WsgState::new(dim, initial_animals);
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_window_dimensions(1000., 700.)
        .setup::<VisState, WsgState>(VisState, state);
    app.add_system(DenseNumberGrid2D::batch_render);
    app.run()
}
