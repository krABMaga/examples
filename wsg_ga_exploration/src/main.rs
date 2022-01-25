use rand::distributions::weighted::WeightedIndex;

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

pub const MUTATION_RATE: f64 = 0.20;
pub const DESIRED_FITNESS: f32 = 0.92;
pub const MAX_GENERATION: u32 = 10;
pub const INDIVIDUALS: u32 = 50;

pub const INITIAL_NUM_WOLVES: u32 = (100. * 0.4) as u32;
pub const INITIAL_NUM_SHEEPS: u32 = (100. * 0.6) as u32;

pub const WIDTH: i32 = 25;
pub const HEIGHT: i32 = 25;
pub const STEP: u64 = 20;
pub const REPETITION: u32 = 2;

fn main() {
    // macro used to execute model exploration using a genetic algorithm
    let result = explore_ga_sequential!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        WsgState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        REPETITION,
    );

    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name = "explore_result".to_string();
        let _res = write_csv(&name, &result);
    }
}

// function that initialize the populatin
fn init_population() -> Vec<String> {
    // create an array of WsgState
    let mut population = Vec::new();

    // create n=POPULATION individuals
    for _ in 0..INDIVIDUALS {
        let mut rng = rand::thread_rng();

        // random initialization within a range
        // and create the individual
        let gain_energy_sheep = rng.gen_range(1.0..=50.0);
        let gain_energy_wolf = rng.gen_range(1.0..=50.0);
        let sheep_repr = rng.gen_range(0.01..=0.2);
        let wolf_repr = rng.gen_range(0.01..=0.2);
        let full_grown = rng.gen_range(10..40);

        population.push(format!(
            "{gain_energy_sheep};{gain_energy_wolf};{sheep_repr};{wolf_repr};{full_grown}"
        ));
    }

    // return the array of paramateres to create individuals
    population
}

fn selection(population_fitness: &mut Vec<(String, f32)>) {
    // weighted tournament selection
    let mut rng = rand::thread_rng();
    let mut len = population_fitness.len();

    // build an array containing the fitness values in order to be used for the
    // weighted selection
    let mut weight = Vec::new();
    for individual_fitness in population_fitness.iter_mut() {
        let mut single_weight = (individual_fitness.1 * 100.).floor() as u32;
        if single_weight == 0 {
            single_weight += 1;
        }
        weight.push(single_weight);
    }

    let mut new_population: Vec<(String, f32)> = Vec::new();

    if len % 2 == 1 {
        len = (len / 2) + 1;
    } else {
        len /= 2;
    }

    for i in 0..len {
        let dist = WeightedIndex::new(&weight).unwrap();

        let idx_one = dist.sample(&mut rng);
        weight[idx_one] = 0;

        if i == len - 1 && population_fitness.len() % 2 == 1 {
            new_population.push(population_fitness[idx_one].clone());
            continue;
        }

        let mut idx_two = idx_one;
        while idx_one == idx_two {
            idx_two = dist.sample(&mut rng);
        }
        weight[idx_two] = 0;

        // choose the individual with the highest fitness
        if population_fitness[idx_one].1 < population_fitness[idx_two].1 {
            new_population.push(population_fitness[idx_two].clone());
        } else {
            new_population.push(population_fitness[idx_one].clone());
        }
    }
    *population_fitness = new_population;
}

fn crossover(population: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    // combine two random parents

    let mut parents = population.clone();

    let mut additional_individuals = INDIVIDUALS as usize - population.len(); // N/2

    while additional_individuals != 0 {
        // select two random individuals
        let idx_one = rng.gen_range(0..parents.len());
        let parent_one = parents[idx_one].clone();
        parents.remove(idx_one);

        if additional_individuals == 1 {
            population.push(parent_one);
            additional_individuals -= 1;
            continue;
        }

        let mut idx_two = rng.gen_range(0..parents.len());
  
        let parent_two = parents[idx_two].clone();
        parents.remove(idx_two);

        let parent_one: Vec<&str> = parent_one.split(';').collect();
        let one_gain_s = parent_one[0];
        let one_gain_w = parent_one[1];
        let one_repr_s = parent_one[2];
        let one_repr_w = parent_one[3];
        let one_grass = parent_one[4];

        let parent_two: Vec<&str> = parent_two.split(';').collect();
        let two_gain_s = parent_two[0];
        let two_gain_w = parent_two[1];
        let two_repr_s = parent_two[2];
        let two_repr_w = parent_two[3];
        let two_grass = parent_two[4];

        //use probs to cross genoms
        let new_individual_one = format!(
            "{};{};{};{};{}",
            if rng.gen_bool(0.5){one_gain_s} else {two_gain_s}, 
            if rng.gen_bool(0.5){one_gain_w} else {two_gain_w}, 
            if rng.gen_bool(0.5){one_repr_s} else {two_repr_s}, 
            if rng.gen_bool(0.5){one_repr_w} else {two_repr_w}, 
            if rng.gen_bool(0.5){one_grass} else {two_grass}
        );
        let new_individual_two = format!(
            "{};{};{};{};{}",
            if rng.gen_bool(0.5){one_gain_s} else {two_gain_s}, 
            if rng.gen_bool(0.5){one_gain_w} else {two_gain_w}, 
            if rng.gen_bool(0.5){one_repr_s} else {two_repr_s}, 
            if rng.gen_bool(0.5){one_repr_w} else {two_repr_w}, 
            if rng.gen_bool(0.5){one_grass} else {two_grass}
        );

        population.push(new_individual_one);
        population.push(new_individual_two);

        additional_individuals -= 2;
    }
}

fn mutation(individual: &mut String) {
    let new_individual: Vec<&str> = individual.split(';').collect();
    let one_gain_s = new_individual[0];
    let one_gain_w = new_individual[1];
    let one_repr_s = new_individual[2];
    let one_repr_w = new_individual[3];
    let one_grass = new_individual[4];
    let mut rng = rand::thread_rng();
    let mut new_ind = String::new();
    // mutate one random parameter
    // randomly increase or decrease one gain
    if rng.gen_bool(MUTATION_RATE) {
        if rng.gen_bool(0.5) {
            // mutate gain sheep
            let x = rng.gen_range(1.0..=50.0_f32);
            let mut new_gain_s = one_gain_s
                .parse::<f32>()
                .expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
                if new_gain_s + x < 50.0 {
                    new_gain_s += x;
                }
            } else if new_gain_s - x > 1. {
                new_gain_s -= x;
            }
            new_ind.push_str(format!("{};{};", new_gain_s, one_gain_w).as_str());
        } else {
            // mutate gain wolf
            let x = rng.gen_range(1.0..=50.0_f32);
            let mut new_gain_w = one_gain_w
                .parse::<f32>()
                .expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
                if new_gain_w + x < 50.0 {
                    new_gain_w += x;
                }
            } else if new_gain_w - x > 1. {
                new_gain_w -= x;
            }
            new_ind.push_str(format!("{};{};", one_gain_s, new_gain_w).as_str());
        }
    } else {
        new_ind.push_str(format!("{};{};", one_gain_s, one_gain_w).as_str());
    }

    if rng.gen_bool(MUTATION_RATE) {
        if rng.gen_bool(0.5) {
            // mutate repr sheep
            let x = rng.gen_range(0.01..=0.1_f32);
            let mut new_repr_s = one_repr_s
                .parse::<f32>()
                .expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
                if new_repr_s + x < 1.0 {
                    new_repr_s += x;
                }
            } else if new_repr_s - x > 0.01 {
                new_repr_s -= x;
            }
            new_ind.push_str(format!("{};{};", new_repr_s, one_repr_w).as_str());
        } else {
            // mutate repr wolf
            let x = rng.gen_range(0.01..=1.0_f32);
            let mut new_repr_w = one_repr_w
                .parse::<f32>()
                .expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
                if new_repr_w + x < 1.0 {
                    new_repr_w += x;
                }
            } else if new_repr_w - x > 0.01 {
                new_repr_w -= x;
            }
            new_ind.push_str(format!("{};{};", one_repr_s, new_repr_w).as_str());
        }
    } else {
        new_ind.push_str(format!("{};{};", one_repr_s, one_repr_w).as_str());
    }

    if rng.gen_bool(MUTATION_RATE) {
        let x = rng.gen_range(5..=10_u16);
        let mut new_grass = one_grass
            .parse::<u16>()
            .expect("Unable to parse str to f32!");
        if rng.gen_bool(0.5) {
            if new_grass + x < 40 {
                new_grass += x;
            }
        } else if new_grass - x > 10 {
            new_grass -= x;
        }
        new_ind.push_str(format!("{}", new_grass).as_str());
    } else {
        new_ind.push_str(format!("{}", one_grass).as_str());
    }

    *individual = new_ind;
}

fn fitness(computed_ind: &mut Vec<(WsgState, Schedule)>) -> f32 {
    let desired_sheeps = 1000.;
    let desired_wolves = 200.;
    let max_agent = 5000.;

    let mut total_avg = 0.;
    let len = computed_ind.len();
    for (state, schedule) in computed_ind {
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
            perc_sheeps =
                1. - (((desired_sheeps - mean_agent).abs()) / (max_agent - desired_sheeps));
        }

        let perc_wolves;
        if mean_agent <= desired_wolves {
            perc_wolves = mean_agent / desired_wolves;
        } else {
            perc_wolves =
                1. - (((desired_wolves - mean_agent).abs()) / (max_agent - desired_wolves));
        }

        let average;

        if num_wolves == 0. || num_sheeps == 0. {
            // println!("Number of animals is zero at step {}", schedule.step);
            average = 0.;
        } else {
            average = (perc_sheeps + perc_wolves) / 2.;
        }
        state.fitness = average;
        total_avg += average;
    }

    total_avg = total_avg / len as f32;
    if total_avg <= 0. {
        0.001
    } else {
        (total_avg * 1000.).round() / 1000.
    }
}
