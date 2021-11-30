mod model;
use crate::model::state::Flocker;


#[cfg(feature = "distributed")] 
#[macro_use]
extern crate memoffset;

#[cfg(feature = "distributed")] 
use {
    mpi::{
        datatype::UserDatatype,
        traits::*,
        Address,
    },
};
// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State,
    rust_ab::*,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization, rust_ab::FrameRow,
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

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);
    let my_rank = world.rank();

    let initial_flockers = vec![
        100,
        200,
        300,
        400,
        1000,
        2000,
        3000,
        4000,
    ];
    let width = vec![
        400., 
        500.,
        600.,
        700.,
        4000., 
        5000.,
        6000.,
        7000.,
    ];
    let height = vec![
        400., 
        500.,
        600.,
        700.,
        4000., 
        5000.,
        6000.,
        7000.,
    ];

    let num_procs = world.size() as u32;
    // explore the result of simulation using initial_animals and dim as input
    // the macro returns a dataframe with the required output
    let result = explore!(
        step, // number of step
        1, // number of repetition of the simulation for each configuration
        Flocker, // name of the state
        input { // input to use to configure the state that will change at each time
            width: f32
            height: f32
            initial_flockers: u32
        },
        output[ // desired output that will be written in the dataframe
        //     survived_wolves: u32
        //     survived_sheeps: u32
        ],
        ExploreMode::Distributed,
        ComputationMode::Distributed,
        my_rank: i32,
        num_procs: u32
    );


    if world.rank() == root_rank {
        let size = width.len();
        // t is thereceiver buffer
        // ex: EXAUSTIVE
        // let mut t = vec![result[0]; (world.size() as usize * width.len() * height.len() * initial_flockers.len())];
        // ex: MATCHED
        // let mut t = vec![result[0]; world.size() as usize * 4];
        // ex: DISTRIBUTED
        let mut t = vec![result[0]; size];

        root_process.gather_into_root(&result[..], &mut t[..]);

        //build csv from all processes
        let name = format!("{}", "result_complex");
        let _res = export_dataframe(&name, &t);

    } else {
        //every proc send to root every row
        root_process.gather_into(&result[..]);
    }
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
