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
    // let initial_flockers = vec![100, 200, 300, 400];
    // let width = vec![400., 500., 600., 700.];
    // let height = vec![400., 500., 600., 700.];

    // let mut width: Vec<f32> = Vec::new();
    // let mut height: Vec<f32> = Vec::new();
    // let mut initial_flockers: Vec<u32> = Vec::new();

    /* test reading from csv
        first parameter is the path to the csv file
        other parameters are the names of the columns
        the macro will create a tuple with an array for each column to match the types
        then you have to assign the values to the variables to pass them to the simulation
    */

    let (width, height, initial_flockers) = load_csv!("data.csv", width: f32, height: f32, initial_flockers: u32);

    // println!("{:?}", initial_flockers);
    // println!("{:?}", width);
    // println!("{:?}", height);

    // explore the result of simulation using some input
    // the macro returns a dataframe with the required output
    // only the master return a usable dataframe
    let dataframe = explore!(
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
        ExploreMode::Matched, //una lista di configurazioni N
        ComputationMode::DistributedMPI,// N/P a ogni processo
        //my_rank: i32,
        //num_procs: u32
    );
    
    if !dataframe.is_empty() {
        //i'm the master 
        //build csv from all processes
        let name = format!("{}", "result_main_00");
        let _res = write_csv(&name, &dataframe);
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
