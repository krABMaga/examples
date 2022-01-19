use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use model::state::EpidemicNetworkState;
mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;

///Initial infected nodes
pub static INIT_EDGES: usize = 2;
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
pub static RECOVERY_CHANCE: f64 = 0.3;
pub static GAIN_RESISTANCE_CHANCE: f64 = 0.2;

pub static INITIAL_IMMUNE: f32 = 0.3;
pub static INITIAL_INFECTED: f32 = 0.1;
pub const NUM_NODES: u32 = 100;

pub const MUTATION_RATE: f64 = 0.05;
pub const DESIRED_FITNESS: f32 = 1.;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 10;

pub const WIDTH: f32 = 150.;
pub const HEIGHT: f32 = 150.;

pub const STEP: u64 = 100;

fn main() {
    let mut positions:Vec<Vec<u32>> = Vec::new();

    for _ in 0..POPULATION{
        positions.push(init_positions());
    }   

    let result = explore_distributed_mpi!(
        STEP,
        2,
        EpidemicNetworkState,
        input_vec {
            positions: [u32; 10] //needs to be a slice since a sized implementation is required
        },
        ExploreMode::Matched,
    );

    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name = "explore_result".to_string();
        let _res = write_csv(&name, &result);
    }
}

fn init_positions() -> Vec<u32>{
    let mut rng = rand::thread_rng();
    let mut positions = vec![0; NUM_NODES as usize];
    let mut immune_counter = 0;
    while immune_counter != (INITIAL_IMMUNE * NUM_NODES as f32) as u32 {
        let node_id = rng.gen_range(0..NUM_NODES);

        if positions[node_id as usize] == 0 {
            positions[node_id as usize] = 1;
            immune_counter += 1;
        }
    }

    let mut infected_counter = 0;
    while infected_counter != (INITIAL_INFECTED * NUM_NODES as f32) as u32 {
        let node_id = rng.gen_range(0..NUM_NODES);

        if positions[node_id as usize] == 0 {
            positions[node_id as usize] = 2;
            infected_counter += 1;
        }
    }

    positions
}