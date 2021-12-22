use crate::model::state::WsgState;
mod model;

// Immutable parameters
// Or default parameters
pub const FULL_GROWN: u16 = 20;
pub const MOMENTUM_PROBABILITY: f64 = 0.8;

// Mutable parameters in the fitness
pub const ENERGY_CONSUME: f64 = 1.0;
pub const GAIN_ENERGY_SHEEP: f64 = 5.0;
pub const GAIN_ENERGY_WOLF: f64 = 13.0;
pub const SHEEP_REPR: f64 = 0.2;
pub const WOLF_REPR: f64 = 0.1;

use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::ExploreMode,
    rust_ab::*,
};

fn main() {
    // Parameters that can change
    let step: u64 = 10;

    // tuples to use in the exploration

    // let initial_animals = vec![
    //     ((200. * 0.6) as u32,
    //     ((400. * 0.6) as u32,
    //     // ((800. * 0.6) as u32, (800. * 0.4) as u32),
    //     // ((1600. * 0.6) as u32, (1600. * 0.4) as u32),
    //     // ((3200.*0.6) as u32, (3200.*0.4) as u32),
    //     // ((6400.*0.6) as u32, (6400.*0.4) as u32)
    // ];

    let initial_sheeps = vec![(200. * 0.6) as u32, (400. * 0.6) as u32];

    let initial_wolves = vec![(200. * 0.4) as u32, (400. * 0.4) as u32];

    // let dim = vec![
    //     (800, 800),
    //     (1600, 1600),
    //     // (3200, 3200),
    //     // (6400, 6400),
    //     // (12800, 12800),
    //     // (25600, 25600)
    // ];
    
    let width = vec![800, 1600];
    let height = vec![800, 1600];

    // model exploration in parallel, same syntax of explore
    let result = explore_distributed_mpi!(
        step,
        3,
        WsgState,
        input { // input to use to configure the state that will change at each time
            width: i32
            height: i32
            initial_sheeps: u32
            initial_wolves: u32
        },
        output[ // desired output that will be written in the dataframe
            survived_wolves: u32
            survived_sheeps: u32
        ],
        ExploreMode::Matched,
    );

    // export the dataframe returned by the model exploration into a csv
    let _ = write_csv("result", &result);
}
