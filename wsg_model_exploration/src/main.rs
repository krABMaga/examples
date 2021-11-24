use crate::model::state::WsgState;
mod model;

// Immutable parameters
// Or default parameters

pub const ENERGY_CONSUME: f64 = 1.0;

pub const FULL_GROWN: u16 = 20;

pub const GAIN_ENERGY_SHEEP: f64 = 5.0;
pub const GAIN_ENERGY_WOLF: f64 = 13.0;

pub const SHEEP_REPR: f64 = 0.2;
pub const WOLF_REPR: f64 = 0.1;

pub const MOMENTUM_PROBABILITY: f64 = 0.8;

// pub const INITIAL_NUM_WOLVES: u32 = (1600. * 0.4) as u32;
// pub const INITIAL_NUM_SHEEPS: u32 = (1600. * 0.6) as u32;

// pub const WIDTH: i32 = 6400;
// pub const HEIGHT: i32 = 6400;
// pub const STEP: u64 = 10;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State,
    rust_ab::ExploreMode, rust_ab::Info, rust_ab::ProgressBar, rust_ab::*, std::time::Duration,
};

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    // Parameters that can change
    let step: u64 = 10;

    // initial.animals.0 is the number of sheeps
    // initial.animals.1 is the number of wolves
    // let initial_animals: (u32, u32) = ((25600.*0.6) as u32, (25600.*0.4) as u32);

    // dim.0 is the width of the field
    // dim.1 is the height of the field
    // let dim: (i32, i32) = (6400, 6400);

    // let state = WsgState::new(
    //     dim,
    //     initial_animals
    // );

    // simulate!(step, state, 1, Info::Verbose);

    // Model exploration

    // tuples to use in the exploration

    let initial_animals = vec![
        ((200. * 0.6) as u32, (200. * 0.4) as u32),
        ((400. * 0.6) as u32, (400. * 0.4) as u32),
        // ((800. * 0.6) as u32, (800. * 0.4) as u32),
        // ((1600. * 0.6) as u32, (1600. * 0.4) as u32),
        // ((3200.*0.6) as u32, (3200.*0.4) as u32),
        // ((6400.*0.6) as u32, (6400.*0.4) as u32)
    ];

    let dim = vec![
        (800, 800),
        (1600, 1600),
        // (3200, 3200),
        // (6400, 6400),
        // (12800, 12800),
        // (25600, 25600)
    ];

    // explore the result of simulation using initial_animals and dim as input
    // the macro returns a dataframe with the required output
    // let result = explore!(
    //     step, // number of step
    //     1, // number of repetition of the simulation for each configuration
    //     WsgState, // name of the state
    //     input { // input to use to configure the state that will change at each time
    //         dim:(i32, i32)
    //         initial_animals:(u32, u32)
    //     },
    //     output[ // desired output that will be written in the dataframe
    //         survived_wolves: u32
    //         survived_sheeps: u32
    //     ],
    //     ExploreMode::Matched
    // );

    // model exploration in parallel, same syntax of explore
    let result = explore_parallel!(
        step,
        1,
        WsgState,
        input { // input to use to configure the state that will change at each time
            dim:(i32, i32)
            initial_animals:(u32, u32)
        },
        output[ // desired output that will be written in the dataframe
            survived_wolves: u32
            survived_sheeps: u32
        ],
        ExploreMode::Matched
    );

    // export the dataframe returned by the model exploration into a csv
    export_dataframe("result", &result);
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
    let dim: (i32, i32) = (25, 25);
    let initial_animals: (u32, u32) = ((100. * 0.6) as u32, (100. * 0.4) as u32);

    let state = WsgState::new(dim, initial_animals);
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
        .with_window_dimensions(1000., 700.)
        .setup::<VisState, WsgState>(VisState, state);
    app.add_system(DenseNumberGrid2D::batch_render.system());
    app.run()
}
