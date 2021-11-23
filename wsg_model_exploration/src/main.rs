use crate::model::state::WsgState;
mod model;

pub const ENERGY_CONSUME: f64 = 1.0;
pub const NUM_WOLVES: u32 = (1600. * 0.4) as u32;
pub const NUM_SHEEPS: u32 = (1600. * 0.6) as u32;
pub const FULL_GROWN: u16 = 20;

pub const GAIN_ENERGY_SHEEP: f64 = 5.0;
pub const GAIN_ENERGY_WOLF: f64 = 13.0;

pub const SHEEP_REPR: f64 = 0.2;
pub const WOLF_REPR: f64 = 0.1;

pub const MOMENTUM_PROBABILITY: f64 = 0.8;

pub const WIDTH: i32 = 6400;
pub const HEIGHT: i32 = 6400;
pub const STEP: u64 = 10;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::Info,
    rust_ab::ProgressBar, rust_ab::*, std::time::Duration,
};

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    //TODO togli ste cose
    simulate!(STEP, WsgState::new(WIDTH, HEIGHT), 1, Info::VERBOSE);

    // let num_wolves = gen_param!(u32, 10, 50, 3);
    // let num_sheeps = gen_param!(u32, 10, 50, 3);
    // let num = vec![
    //     ((800.*0.4) as u32, (800.*0.6) as u32),
    //     ((1600.*0.4) as u32, (1600.*0.6) as u32),
    //     // ((3200.*0.4) as u32, (3200.*0.6) as u32),
    //     // ((6400.*0.4) as u32, (6400.*0.6) as u32)
    // ];

    // // let dim = vec![
    // //     (3200, 3200),
    // //     (6400, 6400),
    // //     (12800, 12800),
    // //     (25600, 25600)
    // // ];

    // let result = explore!(
    //     STEP,
    //     WsgState::new(WIDTH, HEIGHT),
    //     1,
    //     input {
    //         num:(u32, u32)
    //     },
    //     output[
    //         survived_wolves: u32
    //         survived_sheeps: u32
    //     ]
    // );

    // let result = explore_parallel!(
    //     STEP,
    //     1,
    //     WsgState,
    //     param (HEIGHT, WIDTH,),
    //     input {
    //         num_wolves:u32
    //         num_sheeps:u32
    //     },
    //     output[
    //         survived_wolves: u32
    //         survived_sheeps: u32
    //     ]
    // );
    //export_dataframe("result", &result);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem,
    rust_ab::engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
    rust_ab::visualization::fields::number_grid_2d::BatchRender,
    rust_ab::visualization::visualization::Visualization,
};

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = WsgState::new(WIDTH, HEIGHT);
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(1000., 700.)
        .setup::<VisState, WsgState>(VisState, state);
    app.add_system(DenseNumberGrid2D::batch_render.system());
    app.run()
}
