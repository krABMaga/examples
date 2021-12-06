use crate::model::sheep::Sheep;
use crate::model::state::WsgState;
use crate::model::wolf::Wolf;
mod model;

// Immutable parameters
// Or default parameters

pub const MOMENTUM_PROBABILITY: f64 = 0.8;
pub const ENERGY_CONSUME: f32 = 1.0;
pub const MUTATION_RATE: f64 = 0.05;

// Mutable parameters in the fitness
// pub const FULL_GROWN: u16 = 20;
// pub const GAIN_ENERGY_SHEEP: f64 = 5.0;
// pub const GAIN_ENERGY_WOLF: f64 = 13.0;
// pub const SHEEP_REPR: f64 = 0.2;
// pub const WOLF_REPR: f64 = 0.1;

pub const INITIAL_NUM_WOLVES: u32 = (100. * 0.4) as u32;
pub const INITIAL_NUM_SHEEPS: u32 = (100. * 0.6) as u32;

pub const WIDTH: i32 = 20;
pub const HEIGHT: i32 = 20;
pub const STEP: u64 = 10;

use {
    rust_ab::engine::{schedule::Schedule, state::State},
    rust_ab::*,
};

fn main() {
    ga!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        WsgState,
        1000.,
        30,
        STEP
    );
}

fn init_population() -> Vec<WsgState> {
    let mut population = Vec::new();
    // create initial population for the genetic algorithm

    for _ in 0..8 {
        let mut rng = rand::thread_rng();
        // let energy_consume = rng.gen_range(1.0..=2.0);
        let gain_energy_sheep = rng.gen_range(1.0..=50.0);
        let gain_energy_wolf = rng.gen_range(1.0..=50.0);
        let sheep_repr = rng.gen_range(0.01..=0.2);
        let wolf_repr = rng.gen_range(0.0..=0.2);
        let full_grown = rng.gen_range(0..100);

        let state = WsgState::new(
            gain_energy_sheep,
            gain_energy_wolf,
            sheep_repr,
            wolf_repr,
            full_grown,
        );

        population.push(state);
    }

    population
}

fn selection(population: &mut Vec<WsgState>) {
    // sort population for fitness
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    // println!("BEFORE selection - Population len {}", population.len());

    // remove worst half of the population for slice
    population.truncate(population.len() / 2);
    // for i in 0..population.len(){
    // println!("Fitness after {}", population[i].fitness);
    // }
    // println!("AFTER selection - Population len {}", population.len());
}

fn mutation(state: &mut WsgState) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(MUTATION_RATE) {
        // state.energy_consume = rng.gen_range(1.0..=2.0);
    }
    // println!("New energy {}", state.energy_consume);
}

fn crossover(population: &mut Vec<WsgState>) {
    // iterates on population two by two
    // for i in (0..population.len()/2).step_by(2){
    //     let parent_one = &population[i];
    //     let parent_two = &population[i+1];

    //     // println!("Parent one {}", parent_one.fitness);
    //     // println!("Parent two {}", parent_two.fitness);
    //     let energy = (parent_one.energy + parent_two.energy) / 2.;
    //     population[i+1].energy_consume = energy;
    // }
}

fn fitness(state: &mut WsgState, schedule: Schedule) -> f32 {
    // let agents = schedule.get_all_events();
    // let mut sheeps: f32 = 0.;
    // let mut wolves: f32 = 0.;
    // let mut num_sheeps: f32 = 0.;
    // let mut num_wolves: f32 = 0.;

    // for n in agents {
    //     if let Some(s) = n.downcast_ref::<Sheep>() {
    //         sheeps += s.energy;
    //         num_sheeps += 1.;
    //     }
    //     if let Some(w) = n.downcast_ref::<Wolf>() {
    //         wolves += w.energy;
    //         num_wolves += 1.;
    //     }
    // }

    // if sheeps == 0. || wolves == 0. {
    //     state.fitness = 0.;
    //     println!("Zero.");
    //     return 0.;
    // }

    //let fitness = 1. / ((wolves - sheeps).abs() - (INITIAL_NUM_WOLVES as f32 - INITIAL_NUM_SHEEPS as f32).abs()).abs();
    //let fitness = 1. / ((wolves - sheeps).abs() / (INITIAL_NUM_WOLVES as f32 - INITIAL_NUM_SHEEPS as f32).abs());
    // let fitness: f32 = (sheeps + wolves) * (1. / (1. + (num_sheeps - num_wolves).abs()));

    // println!("Fitness: {}", fitness);

    // let fitness = (INITIAL_NUM_WOLVES as f32 - INITIAL_NUM_SHEEPS as f32).abs() / (wolves - sheeps).abs() ;
    // state.fitness = fitness;
    // fitness
    1.
}
