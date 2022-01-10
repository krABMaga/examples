mod model;
use crate::model::state::Flocker;

use {rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::*};

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

fn main() {
    let step = 10;

    /* test reading from csv
        first parameter is the path to the csv file
        other parameters are the names of the columns
        the macro will create a tuple with an array for each column to match the types
        then you have to assign the values to the variables to pass them to the simulation
    */
/* 
    let (width, height, initial_flockers) =
        load_csv!("data.csv", width: f32, height: f32, initial_flockers: u32);
 */
    let width = [ 50., 80.];
    let height  = [50., 80.];
    let initial_flockers = [ 200, 400];

    // explore the result of simulation using some input
    // the macro returns a dataframe with the required output
    // only the master return a usable dataframe
    let dataframe = explore_sequential!(
        step, // number of step
        2, // number of repetition of the simulation for each configuration
        Flocker, // name of the state
        input { // input to use to configure the state that will change at each time
            width: f32
            height: f32
            initial_flockers: u32
        },
        output[ // desired output that will be written in the dataframe
        ],
        ExploreMode::Matched,
    );

    if !dataframe.is_empty() {
        // I'm the master
        // build csv from all processes
        let name = "result_main_00".to_string();
        let _res = write_csv(&name, &dataframe);
    }
}