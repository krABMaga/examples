use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use rand::prelude::*;

use model::state::EpidemicNetworkState;
use std::cmp::Ordering::Equal;
mod model;

// generic model parameters
// pub static DESIRED_RT: f32 = 2.;
// pub static INITIAL_INFECTED: f32 = 0.01;
pub static INIT_EDGES: usize = 1;
pub const NUM_NODES: u32 = 5_000;

// GA specific parameters
pub const MUTATION_RATE: f64 = 0.2;
pub const DESIRED_FITNESS: f32 = 0.;
pub const MAX_GENERATION: u32 = 1000;
pub const INDIVIDUALS: u32 = 120;
pub const REPETITION: u32 = 20;
pub const IS_GA: bool = false;

lazy_static! {
    pub static ref DATA: Vec<f32> = {
        let mut rdr = Reader::from_path("data/data_perc_full.csv").unwrap();

        let mut x: Vec<f32> = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            let y: f32 = record[0].parse().unwrap();
            x.push(y);
        }
        x
    };
    // pub static ref SEED: u64 = rand::thread_rng().gen();
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(1));
}

pub const STEP: u64 = 51;

fn main() {
    if !IS_GA {
        let mut avg_results: Vec<f32> = vec![0.0; 45];
        let parameters = "0.0633509;0.22434065";

        for i in 0..REPETITION as usize {
            println!("Running simulation {}...", i);
            let mut state = EpidemicNetworkState::new_with_parameters(i, parameters);
            simulate!(STEP, &mut state, 1, Info::Verbose);
            for j in 0..45 {
                avg_results[j] += state.weekly_infected[j] / NUM_NODES as f32;
            }
        }

        for j in 0..45 {
            avg_results[j] /= REPETITION as f32;
        }

        let mut ind_error = 0.;
        
        // let mut sum = 0.;
        // let alpha: f32 = 0.1;
        // for k in 0..45 {
        //     let weight = 1. / (alpha * (1. - alpha).powf(k as f32));
        //     ind_error += weight as f32 * ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
        //     sum += weight as f32;
        // }
        // ind_error = (ind_error / (sum * 45.)).sqrt();

        for k in 0..45 {
            ind_error += ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
        }

        let file_name = format!("sim_data_avg.csv");
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .write(true)
            .create(true)
            .open(file_name.to_string())
            .unwrap();

            writeln!(file, "{:#?}", parameters).expect("Unable to write file.");
            writeln!(file, "Error {:#?}", ind_error).expect("Unable to write file.");

        for i in 0..avg_results.len() {
            writeln!(file, "{:#?}", avg_results[i]).expect("Unable to write file.");
        }
        println!("Avg_error: {} ", ind_error);
    } else {
        let result = explore_ga_parallel!(
            init_population,
            fitness,
            selection,
            mutation,
            crossover,
            cmp,
            EpidemicNetworkState,
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

}

fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    // let mut error = 0.;
    // for i in 0..computed_ind.len() {
    //     // iterates through the weekly infected array to normalize it
    //     for j in 0..computed_ind[i].0.weekly_infected.len() {
    //         computed_ind[i].0.weekly_infected[j] /= NUM_NODES as f32;
    //     }

    //     // compute the error of the simulated results compared to the observed data
    //     // using the summation of each value of 61 days
    //     // (simulated[i] - observed[i])^2
    //     // where simulated[i] is the weekly average of new infected of the day i of the simulation
    //     // and observed[i] is the weekly average of new infected of the day i within the official data
    //     let mut ind_error = 0.;
    //     // let alpha = 0.1;
    //     for k in 0..45 {
    //         ind_error +=
    //             //((k+1) as f32).ln() *
    //             // (alpha * (1 - alpha).powf(k-1)) *
    //                 // ((DATA[k] - computed_ind[i].0.weekly_infected[k]).powf(2.));
    //                 ((DATA[k] - computed_ind[i].0.weekly_infected[k]) / DATA[k]).powf(2.);
    //     }
    //     // ind_error = (ind_error/45.).sqrt();
    //     error += ind_error;
    // }
    // error / computed_ind.len() as f32

    let mut avg_results: Vec<f32> = vec![0.0; 45];

    for j in 0..45 {
        for i in 0..computed_ind.len() {
            avg_results[j] += computed_ind[i].0.weekly_infected[j] / NUM_NODES as f32;
        }
        avg_results[j] /= computed_ind.len() as f32;
    }

    let mut ind_error = 0.;

    // let mut sum = 0.;
    // let alpha: f32 = 0.1;
    // for k in 0..45 {
    //     let weight = 1. / (alpha * (1. - alpha).powf(k as f32));
    //     ind_error += weight as f32 * ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
    //     sum += weight as f32;
    // }
    // ind_error = (ind_error / (sum * 45.)).sqrt();

    for k in 0..45 {
        ind_error += ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
    }

    ind_error
}

// we want to minimize the fitness, therefore the comparison
// return true, meaning that fitness1 is better than fitness2,
// if fitness1 is lower than fitness 2
fn cmp(fitness1: &f32, fitness2: &f32) -> bool {
    *fitness1 < *fitness2
}

// function that initialize the populatin
fn init_population() -> Vec<String> {
    // create an array of EpidemicNetworkState
    let mut population = Vec::new();
    let mut rng = RNG.lock().unwrap();
    // create n=INDIVIDUALS individuals
    for _ in 0..INDIVIDUALS {
        // create the individual
        let x = rng.gen_range(0.0..=1.0_f32).to_string(); // spread chance
        let y = rng.gen_range(0.0..=1.0_f32).to_string(); // recovery chance
        population.push(format!("{};{}", x, y));
    }

    // return the array of individuals, i.e. the population (only the parameters)
    population
}

fn selection(population_fitness: &mut Vec<(String, f32)>) {
    // weighted tournament selection
    // let mut rng = rand::thread_rng();
    // let mut len = population_fitness.len();

    // // build an array containing the fitness values in order to be used for the
    // // weighted selection
    // let mut weight = Vec::new();
    // for individual_fitness in population_fitness.iter_mut() {
    //     let mut single_weight: f32;
    //     if individual_fitness.1 != 0. {
    //         single_weight = 0.1 / individual_fitness.1; // 1 / 0,001 = 1000
    //     } else {
    //         println!("----------------------- test max");
    //         single_weight = f32::MAX;
    //     }

    //     if single_weight == 0. {
    //         single_weight += 1.;
    //     }
    //     weight.push(single_weight);
    // }

    // let mut new_population: Vec<(String, f32)> = Vec::new();

    // if len % 2 == 1 {
    //     len = (len / 2) + 1;
    // } else {
    //     len /= 2;
    // }

    // for i in 0..len {
    //     let dist = WeightedIndex::new(&weight).unwrap();

    //     let idx_one = dist.sample(&mut rng);
    //     weight[idx_one] = 0.;

    //     if i == len - 1 && population_fitness.len() % 2 == 1 {
    //         new_population.push(population_fitness[idx_one].clone());
    //         continue;
    //     }

    //     let mut idx_two = idx_one;
    //     while idx_one == idx_two {
    //         idx_two = dist.sample(&mut rng);
    //     }
    //     weight[idx_two] = 0.;

    //     // choose the individual with the highest fitness
    //     if population_fitness[idx_one].1 < population_fitness[idx_two].1 {
    //         new_population.push(population_fitness[idx_two].clone());
    //     } else {
    //         new_population.push(population_fitness[idx_one].clone());
    //     }
    // }
    // *population_fitness = new_population;

    // sort the population based on the fitness
    population_fitness.sort_by(|s1, s2| s1.1.partial_cmp(&s2.1).unwrap_or(Equal));
}

fn crossover(population: &mut Vec<String>) {
    let mut children: Vec<String> = Vec::new();

    let twenty_perc = INDIVIDUALS as f32 * 0.2;
    for i in 0..(twenty_perc as usize) {
        children.push(population[i].clone());
    }

    let children_num = INDIVIDUALS as f32 - twenty_perc;

    if population.len() == 0 {
        panic!("Population len can't be 0");
    }

    for _ in 0..(children_num as usize) {
        // select two random individuals

        let idx_one = RNG.lock().unwrap().gen_range(0..population.len());
        let parent_one = population[idx_one].clone();
        let mut idx_two = RNG.lock().unwrap().gen_range(0..population.len());
        while idx_one == idx_two {
            idx_two = RNG.lock().unwrap().gen_range(0..population.len());
        }
        let parent_two = population[idx_two].clone();

        // take the parameters of parent_one
        let parent_one: Vec<&str> = parent_one.split(';').collect();
        let one_spread = parent_one[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let one_recovery = parent_one[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");

        // take the parameters of parent_two
        let parent_two: Vec<&str> = parent_two.split(';').collect();
        let two_spread = parent_two[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let two_recovery = parent_two[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");

        let mut min;
        let mut max;
        let alpha = 0.1;

        // new spread
        if one_spread <= two_spread {
            min = one_spread;
            max = two_spread;
        } else {
            min = two_spread;
            max = one_spread;
        }
        let mut range = max - min;
        let mut p_min = 0.;
        let mut p_max = 1.;
        if min - (range * alpha) > 0. {
            p_min = min - (range * alpha);
        }
        if max + (range * alpha) < 1.0 {
            p_max = max + (range * alpha);
        }
        if p_min >= p_max {
            p_min = 0.;
            p_max = 1.;
        }
        let new_spread = RNG.lock().unwrap().gen_range(p_min..=p_max);

        // new_recovery
        if one_recovery <= two_recovery {
            min = one_recovery;
            max = two_recovery;
        } else {
            min = two_recovery;
            max = one_recovery;
        }
        range = max - min;
        p_min = 0.;
        p_max = 1.0;
        if min - (range * alpha) > 0. {
            p_min = min - (range * alpha);
        }
        if max + (range * alpha) < 1.0 {
            p_max = max + (range * alpha);
        }
        if p_min >= p_max {
            p_min = 0.;
            p_max = 1.;
        }
        let new_recovery = RNG.lock().unwrap().gen_range(p_min..=p_max);

        let new_individual = format!(
            "{};{}",
            new_spread, new_recovery
        );
        children.push(new_individual);
    }

    *population = children;

    // let len = population.len();
    // while additional_individuals > 0 {
    //     // select two random individuals
    //     let idx_one = RNG.lock().unwrap().gen_range(0..len);
    //     //let parent_one = parents[idx_one].clone();
    //     let parent_one = population[idx_one].clone();
    //     //parents.remove(idx_one);

    //     // if additional_individuals == 1 {
    //     //     population.push(parent_one);
    //     //     additional_individuals -= 1;
    //     //     continue;
    //     // }

    //     let mut idx_two = rng.gen_range(0..len);

    //     while idx_one == idx_two {
    //         idx_two = rng.gen_range(0..len);
    //     }

    //     let parent_two = population[idx_two].clone();

    //     let parent_one: Vec<&str> = parent_one.split(';').collect();
    //     let one_spread = parent_one[0]
    //         .parse::<f32>()
    //         .expect("Unable to parse str to f32!");
    //     let one_recovery = parent_one[1]
    //         .parse::<f32>()
    //         .expect("Unable to parse str to f32!");

    //     let parent_two: Vec<&str> = parent_two.split(';').collect();
    //     let two_spread = parent_two[0]
    //         .parse::<f32>()
    //         .expect("Unable to parse str to f32!");
    //     let two_recovery = parent_two[1]
    //         .parse::<f32>()
    //         .expect("Unable to parse str to f32!");

    //     if one_spread == two_spread || one_recovery == two_recovery {
    //         continue;
    //     }

    //     let new_individual = format!(
    //         "{};{}",
    //         (one_spread + two_spread) / 2.,
    //         (one_recovery + two_recovery) / 2.
    //     );

    //     population.push(new_individual);

    //     additional_individuals -= 1;
    // }
}

fn mutation(individual: &mut String) {
    let new_ind: String;
    let new_individual: Vec<&str> = individual.split(';').collect();
    let one_spread = new_individual[0];
    let one_recovery = new_individual[1];

    // mutate one random parameter
    // randomly increase or decrease spread orrecovery
    if RNG.lock().unwrap().gen_bool(MUTATION_RATE) {
        // mutate spread
        let mut new_spread = one_spread
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let alpha = 0.1;
        let mut min = if new_spread - alpha < 0. {
            0.
        } else {
            new_spread - alpha
        };
        let mut max = if new_spread + alpha > 1. {
            1.
        } else {
            new_spread + alpha
        };
        if min >= max {
            min = 0.;
            max = 1.;
        }
        new_spread = RNG.lock().unwrap().gen_range(min..=max);

        let mut new_recovery = one_recovery
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        min = if new_recovery - alpha < 0. {
            0.
        } else {
            new_recovery - alpha
        };
        max = if new_recovery + alpha > 1. {
            1.
        } else {
            new_recovery + alpha
        };
        if min >= max {
            min = 0.;
            max = 1.;
        }
        new_recovery = RNG.lock().unwrap().gen_range(min..=max);

        new_ind = format!(
            "{};{}",
            new_spread, new_recovery
        );
        *individual = new_ind;
    }
}
