use rand::distributions::weighted::WeightedIndex;
use rand::seq::SliceRandom;

use rust_ab::{
    engine::{schedule::Schedule, state::State},
    *,
};

use crate::model::sheep::Sheep;
use crate::model::state::WsgState;
use crate::model::wolf::Wolf;
mod model;

// Immutable parameters
// Or default parameters
pub const MOMENTUM_PROBABILITY: f64 = 0.8;
pub const ENERGY_CONSUME: f32 = 1.0;

pub const MUTATION_RATE: f64 = 0.05;
pub const DESIRED_FITNESS: f32 = 0.92;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 10;

pub const INITIAL_NUM_WOLVES: u32 = (100. * 0.4) as u32;
pub const INITIAL_NUM_SHEEPS: u32 = (100. * 0.6) as u32;

pub const WIDTH: i32 = 25;
pub const HEIGHT: i32 = 25;
pub const STEP: u64 = 20;

fn main() {
    // macro used to execute model exploration using a genetic algorithm
    let result = explore_ga_distributed_mpi!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        WsgState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        parameters{
            gain_energy_sheep: f32
            gain_energy_wolf: f32
            sheep_repr: f32
            wolf_repr: f32
            full_grown: u16
        }
    );

    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name = "explore_result".to_string();
        let _res = write_csv(&name, &result);
    }
}

// function that initialize the populatin
fn init_population() -> Vec<WsgState> {
    // create an array of WsgState
    let mut population = Vec::new();

    // create n=POPULATION individuals
    for _ in 0..POPULATION {
        let mut rng = rand::thread_rng();

        // random initialization within a range
        let gain_energy_sheep = rng.gen_range(1.0..=50.0);
        let gain_energy_wolf = rng.gen_range(1.0..=50.0);
        let sheep_repr = rng.gen_range(0.01..=0.2);
        let wolf_repr = rng.gen_range(0.0..=0.2);
        let full_grown = rng.gen_range(10..40);

        // create the individual
        let state = WsgState::new(
            gain_energy_sheep,
            gain_energy_wolf,
            sheep_repr,
            wolf_repr,
            full_grown,
        );

        population.push(state);
    }

    // return the array of individuals, i.e. the population
    population
}

fn selection(population: &mut Vec<WsgState>) {
    // weighted tournament selection
    let mut rng = rand::thread_rng();
    let mut len = population.len();

    // build an array containing the fintess values in order to be used for the
    // weighted selection

    let mut weight = Vec::new();
    for individual in population.iter_mut() {
        weight.push((individual.fitness * 100.).floor() as u32);
    }

    len /= 2;
    // iterate through the population
    for _ in 0..len {
        let dist = WeightedIndex::new(&weight).unwrap();
        let parent_idx_one = dist.sample(&mut rng);
        let parent_idx_two;

        if parent_idx_one == 0 {
            parent_idx_two = parent_idx_one + 1;
        } else {
            parent_idx_two = parent_idx_one - 1;
        }

        // choose the individual with the highest fitness
        // removing the one with the lowest fitness from the population
        if population[parent_idx_one].fitness < population[parent_idx_two].fitness {
            population.remove(parent_idx_one);
            weight.remove(parent_idx_one);
        } else {
            population.remove(parent_idx_two);
            weight.remove(parent_idx_two);
        }
    }
}

fn crossover(population: &mut Vec<WsgState>) {
    let mut rng = rand::thread_rng();

    let additional_individuals = POPULATION as usize - population.len();
    // iterate through the population
    for _ in 0..additional_individuals {
        // select two random individuals
        let mut idx_one = rng.gen_range(0..population.len());
        let idx_two = rng.gen_range(0..population.len());
        while idx_one == idx_two {
            idx_one = rng.gen_range(0..population.len());
        }

        // combines random parameters of the parents
        let parents = vec![idx_one, idx_two];

        // to create a new individual
        let new_individual = WsgState::new(
            population[*(parents.choose(&mut rng).unwrap())].gain_energy_sheep,
            population[*(parents.choose(&mut rng).unwrap())].gain_energy_wolf,
            population[*(parents.choose(&mut rng).unwrap())].sheep_repr,
            population[*(parents.choose(&mut rng).unwrap())].wolf_repr,
            population[*(parents.choose(&mut rng).unwrap())].full_grown,
        );

        // add the new individual to the population
        population.push(new_individual);
    }
}

fn mutation(state: &mut WsgState) {
    let mut rng = rand::thread_rng();

    // mutate one random parameter with assigning random value
    if rng.gen_bool(MUTATION_RATE) {
        let switch = rng.gen_range(0..5);
        match switch {
            0 => state.gain_energy_sheep = rng.gen_range(1.0..=50.0),
            1 => state.gain_energy_wolf = rng.gen_range(1.0..=50.0),
            2 => state.sheep_repr = rng.gen_range(0.01..=0.2),
            3 => state.wolf_repr = rng.gen_range(0.0..=0.2),
            4 => state.full_grown = rng.gen_range(10..40),
            _ => panic!("Invalid mutation switch"),
        }
    }
}

fn fitness(state: &mut WsgState, schedule: Schedule) -> f32 {
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

    let perc_sheeps;
    if mean_agent <= desired_sheeps {
        perc_sheeps = mean_agent / desired_sheeps;
    } else {
        perc_sheeps = 1. - (((desired_sheeps - mean_agent).abs()) / (max_agent - desired_sheeps));
    }

    let perc_wolves;
    if mean_agent <= desired_wolves {
        perc_wolves = mean_agent / desired_wolves;
    } else {
        perc_wolves = 1. - (((desired_wolves - mean_agent).abs()) / (max_agent - desired_wolves));
    }

    let average;

    if num_wolves == 0. || num_sheeps == 0. {
        // println!("Number of animals is zero at step {}", schedule.step);
        average = 0.;
    } else {
        average = (perc_sheeps + perc_wolves) / 2.;
    }

    state.fitness = average;
    average
}
