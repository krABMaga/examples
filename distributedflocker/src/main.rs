use crate::model::state::Flocker;
use mpi::traits::*;
mod model;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::simulate,
    rust_ab::ComputationMode, rust_ab::ExploreMode, rust_ab::Info, rust_ab::ProgressBar,
    rust_ab::*, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// pub static WIDTH: f32 = 200.;
// pub static HEIGHT: f32 = 200.;
// pub static NUM_AGENTS: u32 = 100;

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step = 10;

    let dim = (12800., 12800.);
    let num_agents = 153600;
    let state = Flocker::new(dim, num_agents);
    simulate!(step, state, 1, Info::Normal);

    // let universe = mpi::initialize().unwrap();
    // let world = universe.world();
    // let root_rank = 0;
    // let root_process = world.process_at_rank(root_rank);

    // let step = 10;

    // let initial_flockers = vec![
    //     100,
    //     200,
    // ];

    // let dim = vec![
    //     (100., 100.),
    //     (150., 150.),
    // ];

    // // explore the result of simulation using initial_animals and dim as input
    // // the macro returns a dataframe with the required output
    // let result = explore!(
    //     step, // number of step
    //     1, // number of repetition of the simulation for each configuration
    //     Flocker, // name of the state
    //     input { // input to use to configure the state that will change at each time
    //         dim:(f32, f32)
    //         initial_flockers: u32
    //     },
    //     output[ // desired output that will be written in the dataframe
    //     //     survived_wolves: u32
    //     //     survived_sheeps: u32
    //     ],
    //     ExploreMode::Matched,
    //     ComputationMode::Local
    // );

    // // build the name of the csv for each process
    // let name = format!("{}_{}", "result", world.rank());
    // export_dataframe(&name, &result);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let dim = (200., 200.);
    let num_agents = 100;
    let state = Flocker::new(dim, num_agents);
    Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::rgb(0., 0., 0.))
        .with_name("Flockers")
        .start::<VisState, Flocker>(VisState, state);
}
