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
pub const NUM_NODES: u32 = 5_000;
pub static INIT_EDGES: usize = 2;
pub static INITIAL_INFECTED: f32 = 0.01;
pub static DESIRED_RT: f32 = 2.;

// GA specific parameters
pub const MUTATION_RATE: f64 = 0.2;
pub const DESIRED_FITNESS: f32 = 2.;
pub const MAX_GENERATION: u32 = 100;
pub const POPULATION: u32 = 200;

pub const WIDTH: f32 = 150.;
pub const HEIGHT: f32 = 150.;

pub const STEP: u64 = 30;

fn main() {

    //  let epidemic_network = EpidemicNetworkState::new_with_parameters("0.15;0.02");

    //  simulate!(STEP, epidemic_network, 1, Info::Verbose);
    
    let result = explore_ga_parallel!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        EpidemicNetworkState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        50,
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
        let x = rng.gen_range(0.0..=1.0_f32).to_string(); // spread chance
        let y = rng.gen_range(0.14..=0.3_f32).to_string(); // recovery chance

        population.push(format!("{};{}", x, y));
    }

    // return the array of individuals, i.e. the population (only the parameters)
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

    len /= 2;
    // iterate through the population
    for _ in 0..len {
        let dist = WeightedIndex::new(&weight).unwrap();
        let idx_one = dist.sample(&mut rng);
        let mut idx_two = idx_one;
        while idx_one == idx_two {
            idx_two = dist.sample(&mut rng);
        }

        // choose the individual with the highest fitness
        // removing the one with the lowest fitness from the population
        if population_fitness[idx_one].1 < population_fitness[idx_two].1 {
            population_fitness.remove(idx_one);
            weight.remove(idx_one);
        } else {
            population_fitness.remove(idx_two);
            weight.remove(idx_two);
        }
    }
}

fn crossover(population: &mut Vec<String>) {
    let mut rng = rand::thread_rng();

    // combine one gene of two random parents
    //  a = xa, ya - b = xb, yb
    //  child can be (xa, yb) or (xb, ya)

    let additional_individuals = population.len(); // N/2

    // iterate through the population
    for _ in 0..additional_individuals {
        let new_individual: String;

        // select two random individuals
        let mut idx_one = rng.gen_range(0..population.len()-1);
        let idx_two = rng.gen_range(0..population.len()-1);
        while idx_one == idx_two {
            idx_one = rng.gen_range(0..population.len()-1);
        }

        // combines random parameters of the parents
        let parent_one = population[idx_one].clone();
        let parent_two = population[idx_two].clone();

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
    // mutate one random parameter
    // randomly increase or decrease spread orrecovery
    if rng.gen_bool(MUTATION_RATE) {
        if rng.gen_bool(0.5) {
            // mutate spread
            let x = rng.gen_range(0.01..=0.2_f32);
            let mut new_spread = one_spread.parse::<f32>().expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
               if  new_spread + x < 1.0 {
                   new_spread += x;
               }
            } else {
               if new_spread - x > 0. {
                   new_spread -= x;
               } 
            }
            new_ind = format!("{};{}", new_spread, one_recovery);
        } else {
            // mutate recovery
            let x = rng.gen_range(0.01..=0.2_f32);
            let mut new_recovery = one_recovery.parse::<f32>().expect("Unable to parse str to f32!");
           
            if rng.gen_bool(0.5) {
                if new_recovery + x < 0.3 {
                    new_recovery += x;
                }
             } else {
                if new_recovery - x > 0.14 {
                    new_recovery -= x;
                } 
             }
            new_ind = format!("{};{}", one_spread, new_recovery);
        }
        *individual = new_ind;
    }
}

fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    
    // Sort the array using the RT
    // computed_ind.sort_by(|s1, s2| s1.0.rt.partial_cmp(&s2.0.rt).unwrap_or(Equal));

    // println!("Sorted RT: -------------------------------");
    // for i in 0..computed_ind.len() {
    //     print!("{}\t", computed_ind[i].0.rt);
    // }
    // println!("\n-------------------------------");

    // Get the median of the array
    // let index = (computed_ind.len() + 1) / 2;
    // let mut median = computed_ind[index - 1].0.rt;
    // median  =
    //      median * ( 1. - (30. - computed_ind[index - 1].1.step as f32) / 30.);

    // println!("Fitness RT is {:?}", computed_ind[index - 1].0.rt);

    // 1. - (DESIRED_RT - median).abs() / (DESIRED_RT.powf(2.) + median.powf(2.)).sqrt()


    let mut sum_rt = 0.;
    let mut rt_norm;
    for i in 0..computed_ind.len(){
        rt_norm = computed_ind[i].0.rt * ( 1. - (30. - computed_ind[i].1.step as f32) / 30.);
        sum_rt += rt_norm;
    }
    
    let avg = sum_rt / computed_ind.len() as f32;
    // println!("Fitness: RT is {:?}", avg);

    1. - (DESIRED_RT - avg).abs() / (DESIRED_RT.powf(2.) + avg.powf(2.)).sqrt()
}
