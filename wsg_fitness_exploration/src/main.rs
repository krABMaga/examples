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

pub const WIDTH: i32 = 50;
pub const HEIGHT: i32 = 50;
pub const STEP: u64 = 100;

use {
    rust_ab::engine::{schedule::Schedule, state::State},
    rust_ab::*,
    rust_ab::rand::*,
};
use rand::seq::SliceRandom;

fn main() {
    ga!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        WsgState,
        0.98,
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
        let full_grown = rng.gen_range(10..40);

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
    // tournament selection
    
    let len = population.len();
    let mut rng = rand::thread_rng();

    // iterate through the population
    for _ in 0..len/2{
        // select two random individuals
        let mut idx_one = rng.gen_range(0..population.len());
        let mut idx_two = rng.gen_range(0..population.len());
        while idx_one == idx_two {
            idx_one = rng.gen_range(0..population.len());
        }

        // choose the individuals with the highest fitness
        if population[idx_one].fitness < population[idx_two].fitness {
            population.remove(idx_one);
        } else {
            population.remove(idx_two);
        }
    }
}

fn mutation(state: &mut WsgState) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(MUTATION_RATE) {
        let switch = rng.gen_range(0..5);
        match switch {
            0 => state.gain_energy_sheep = rng.gen_range(1.0..=50.0),
            1 => state.gain_energy_wolf = rng.gen_range(1.0..=50.0),
            2 => state.sheep_repr = rng.gen_range(0.01..=0.2),
            3 => state.wolf_repr = rng.gen_range(0.0..=0.2),
            4 => state.full_grown = rng.gen_range(10..40),
            _ => panic!("Invalid mutation switch")
        }
    }
}

fn crossover(population: &mut Vec<WsgState>) {
    // tournament selection

    let len = population.len();
    let mut rng = rand::thread_rng();

    // iterate through the population
    for _ in 0..len{
        // select two random individuals
        let mut idx_one = rng.gen_range(0..population.len());
        let mut idx_two = rng.gen_range(0..population.len());
        while idx_one == idx_two {
            idx_one = rng.gen_range(0..population.len());
        }

        // combines random parameters of the parents
        let parents = vec![idx_one, idx_two];

        // to create a new individual
        let mut new_individual = WsgState::new(
            population[*(parents.choose(&mut rng).unwrap())].gain_energy_sheep,
            population[*(parents.choose(&mut rng).unwrap())].gain_energy_wolf,
            population[*(parents.choose(&mut rng).unwrap())].sheep_repr,
            population[*(parents.choose(&mut rng).unwrap())].wolf_repr,
            population[*(parents.choose(&mut rng).unwrap())].full_grown, 
        );
        
        // add the new individual to the population
        population.push(new_individual);
    }
    
    println!("Population len after crossover {}", population.len());
}

fn fitness(state: &mut WsgState, schedule: Schedule) -> f32 {

    /*
    D = desired population

    PERC(wolf|sheep) = 
    if mean(agent) <= D(agent)
        media(agent) / D(agent)
    else
        1 - ( (abs(D(agent) - mean(agent))) / MAX_AGENT - D(agent) )

    AVG =
    if wolf = 0 || sheep = 0 
        0
    else
        (PERC(wolf) + PERC(sheep)) / 2
    */

    let desired_sheeps = 1000.;
    let desired_wolves = 200.;
    let max_agent = 5000.;

    let agents = schedule.get_all_events();
    let mut num_sheeps: f32 = 0.;
    let mut num_wolves: f32 = 0.;

    for n in agents {
        if let Some(_s) = n.downcast_ref::<Sheep>() {
            num_sheeps += 1.;
        }
        if let Some(_w) = n.downcast_ref::<Wolf>() {
            num_wolves += 1.;
        }
    }

    let mean_agent = (num_sheeps + num_wolves) / 2.;

    let mut perc_sheeps = 0.;
    if mean_agent <= desired_sheeps{
        perc_sheeps = mean_agent / desired_sheeps;
    } else {
        perc_sheeps = 1. - ( ((desired_sheeps - mean_agent).abs()) / (max_agent - desired_sheeps));
    }

    let mut perc_wolves = 0.;
    if mean_agent <= desired_wolves{
        perc_wolves = mean_agent / desired_wolves;
    } else {
        perc_wolves = 1. - ( ((desired_wolves - mean_agent).abs()) / (max_agent - desired_wolves));
    }

    let mut average;
    
    if num_wolves == 0. || num_sheeps == 0.{
        average = 0.;
    } else {
        average = (perc_sheeps + perc_sheeps) / 2.;
    }

    state.fitness = average;
    average
}
