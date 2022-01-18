use rand::distributions::weighted::WeightedIndex;

use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use std::cmp::Ordering::Equal;

use model::state::EpidemicNetworkState;
mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;

// generic model parameters
pub const NUM_NODES: u32 = 50;
pub static INIT_EDGES: usize = 2;
pub static INITIAL_INFECTED: f32 = 0.1;

// GA specific parameters
pub const MUTATION_RATE: f64 = 0.05;
pub const CROSSOVER_RATE: f64 = 0.5;
pub const DESIRED_FITNESS: f32 = 1.;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 200;

pub const WIDTH: f32 = 150.;
pub const HEIGHT: f32 = 150.;

pub const STEP: u64 = 360;

fn main() {
    let result = explore_ga_sequential!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        EpidemicNetworkState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        10,
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
    // create an array of EpidemicNetworkState
    let mut population = Vec::new();

    // create n=POPULATION individuals
    for _ in 0..POPULATION {
        // create the individual
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.3..=0.3_f32).to_string(); // spread chance
        let y = rng.gen_range(0.3..=0.3_f32).to_string(); // recovery chance

        population.push(format!("{};{}", x, y));
    }

    // return the array of individuals, i.e. the population (only the parameters)
    population
}

fn selection(population_fitness: &mut Vec<(String, f32)>) {
    // weighted tournament selection
    let mut rng = rand::thread_rng();
    let mut len = population_fitness.len();

    // build an array containing the fintess values in order to be used for the
    // weighted selection

    let mut weight = Vec::new();
    for individual_fitness in population_fitness.iter_mut() {
        weight.push((individual_fitness.1 * 100.).floor() as u32);
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
        if population_fitness[parent_idx_one].1 < population_fitness[parent_idx_two].1 {
            population_fitness.remove(parent_idx_one);
            weight.remove(parent_idx_one);
        } else {
            population_fitness.remove(parent_idx_one);
            weight.remove(parent_idx_two);
        }
    }
}

fn crossover(population: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    // combine one gene of two random parents
    //  a = xa, ya - b = xb, yb
    //  child can be (xa, yb) or (xb, ya)

    let additional_individuals = POPULATION as usize - population.len();

    // iterate through the population
    for _ in 0..additional_individuals {
        let new_individual: String;

        // select two random individuals
        let mut idx_one = rng.gen_range(0..population.len());
        let idx_two = rng.gen_range(0..population.len());
        while idx_one == idx_two {
            idx_one = rng.gen_range(0..population.len());
        }

        // combines random parameters of the parents
        let parent_one = population[idx_one].clone();
        let parent_two = population[idx_two].clone();

        // TODO how to use crossover_rate
        // if rng.gen_bool(CROSSOVER_RATE){

        // } else {

        // }

        let parent_one: Vec<&str> = parent_one.split(';').collect();
        let one_spread = parent_one[0];
        let one_recovery = parent_one[1];

        let parent_two: Vec<&str> = parent_two.split(';').collect();
        let two_spread = parent_two[0];
        let two_recovery = parent_two[1];

        if rng.gen_bool(0.5) {
            new_individual = format!("{};{}", one_spread, two_recovery);
        } else {
            new_individual = format!("{};{}", two_spread, one_recovery);
        }

        population.push(new_individual);
    }
}

fn mutation(individual: &mut String) {
    let new_ind: String;

    let new_individual: Vec<&str> = individual.split(';').collect();
    let one_spread = new_individual[0];
    let one_recovery = new_individual[1];

    let mut rng = rand::thread_rng();
    // mutate one random parameter with assigning random value
    if rng.gen_bool(MUTATION_RATE) {
        if rng.gen_bool(0.5) {
            // mutate spread
            let new_spread = rng.gen_range(0.3..=0.3_f32).to_string();
            new_ind = format!("{};{}", new_spread, one_recovery);
        } else {
            // mutate recovery
            let new_recovery = rng.gen_range(0.3..=0.3_f32).to_string();
            new_ind = format!("{};{}", one_spread, new_recovery);
        }
        *individual = new_ind;
    }
}

fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    // Sort the array using the RT

    computed_ind.sort_by(|s1, s2| s1.0.rt.partial_cmp(&s2.0.rt).unwrap_or(Equal));

    // println!("Sorted RT: -------------------------------");
    // for i in 0..computed_ind.len() {
    //     print!("{}\t", computed_ind[i].0.rt);
    // }
    // println!("\n-------------------------------");

    // Get the median of the array
    let index = (computed_ind.len() + 1) / 2;
    let median = computed_ind[index - 1].0.rt;

    // println!("Median is {:?}", computed_ind[index - 1].0.rt);

    let desired_rt: f32 = 3.5;
    1. - (desired_rt - median).abs() / (desired_rt.powf(2.) + median.powf(2.)).sqrt()
}
