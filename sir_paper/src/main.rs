use rand::distributions::weighted::WeightedIndex;

use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use model::state::EpidemicNetworkState;
use std::cmp::Ordering::Equal;
// use std::io::Write;
mod model;



// generic model parameters
// pub static DESIRED_RT: f32 = 2.;
// pub static INITIAL_INFECTED: f32 = 0.01;
pub static INIT_EDGES: usize = 1;
pub const NUM_NODES: u32 = 10_000;

// GA specific parameters
pub const MUTATION_RATE: f64 = 0.3;
pub const DESIRED_FITNESS: f32 = 0.;
pub const MAX_GENERATION: u32 = 50;
pub const INDIVIDUALS: u32 = 60;
pub const REPETITION: u32 = 10;

lazy_static! {
    pub static ref DATA: Vec<f32> = {
        let mut rdr = Reader::from_path("data/data_perc.csv").unwrap();

        let mut x: Vec<f32> = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            let y: f32 = record[0].parse().unwrap();
            x.push(y);
        }
        x
    };
}

pub const STEP: u64 = 37;

fn main() {
    let mut avg_results: Vec<f32> = vec![0.0; 31];

    for i in 0..REPETITION as usize {
        println!("Running simulation {}...", i);
        let mut state =
            EpidemicNetworkState::new_with_parameters(i, "0.078856565;0.1320176;0.33330417;23"); //0.04851025 with Pcg64
        simulate!(STEP, &mut state, 1, Info::Verbose);
        let mut schedule: Schedule = Schedule::new();
        state.init(&mut schedule);
        for i in 0..STEP {
            schedule.step(&mut state);
            if state.end_condition(&mut schedule) {
                break;
            }
        }
        for j in 0..31 {
            avg_results[j] += state.weekly_infected[j] / NUM_NODES as f32;
        }
    }

    for j in 0..31 {
        avg_results[j] /= REPETITION as f32;
    }

    let mut ind_error = 0.;
    let mut sum = 0.;
    let alpha: f32 = 0.05;
    for k in 0..31 {
        let weight = 1. / (alpha * (1. - alpha).powf(k as f32));
        ind_error += weight as f32 * ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
        sum += weight as f32;
    }
    ind_error = (ind_error / (sum * 31.)).sqrt();

    let file_name = format!("sim_data_avg.csv");
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .write(true)
        .create(true)
        .open(file_name.to_string())
        .unwrap();

    for i in 0..avg_results.len() {
        writeln!(file, "{:#?}", avg_results[i]).expect("Unable to write file.");
    }
    writeln!(file, "Error {:#?}", ind_error).expect("Unable to write file.");
    println!("Avg_error: {} ", ind_error);

    // let result = explore_ga_parallel!(
    //     init_population,
    //     fitness,
    //     selection,
    //     mutation,
    //     crossover,
    //     cmp,
    //     EpidemicNetworkState,
    //     DESIRED_FITNESS,
    //     MAX_GENERATION,
    //     STEP,
    //     REPETITION,
    // );
    // if !result.is_empty() {
    //     // I'm the master
    //     // build csv from all procexplore_result
    //     let name = "explore_result".to_string();
    //     let _res = write_csv(&name, &result);
    // }
}

fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    let mut error = 0.;
    for i in 0..computed_ind.len() {
        // iterates through the weekly infected array to normalize it
        for j in 0..computed_ind[i].0.weekly_infected.len() {
            computed_ind[i].0.weekly_infected[j] /= NUM_NODES as f32;
        }

        // compute the error of the simulated results compared to the observed data
        // using the summation of each value of 61 days
        // (simulated[i] - observed[i])^2
        // where simulated[i] is the weekly average of new infected of the day i of the simulation
        // and observed[i] is the weekly average of new infected of the day i within the official data
        let mut ind_error = 0.;
        for k in 0..31 {
            ind_error += ((k+1) as f32).ln() * 
                    ((DATA[k] - computed_ind[i].0.weekly_infected[k]).powf(2.)) ;
                    // ((DATA[k] - computed_ind[i].0.weekly_infected[k]) / DATA[k]).powf(2.) ;
        }
        ind_error = (ind_error/31.).sqrt();
        error += ind_error;
    }
    error / computed_ind.len() as f32

    // let mut average_error = 0.;
    // let N = computed_ind.len().clone();
    // for run in computed_ind {
    //     let mut avg_err = 0.;
    //     // iterates through the weekly infected array to normalize it
    //     // and compute the average
    //     for day in &run.0.weekly_infected {
    //         avg_err += day / NUM_NODES as f32;
    //     }
    //     avg_err /= run.0.weekly_infected.len() as f32;

    //     let mut SST = 0.;
    //     for day in &run.0.weekly_infected {
    //         SST += (day - avg_err).powf(2.);
    //     }
       
    //     let mut SSR = 0.;
    //     for i in 0..31 {
    //         SSR += (run.0.weekly_infected[i] - DATA[i]).powf(2.);
    //     }

    //     let COD = (1. - (SSR / SST)) * 100.; 
    //     average_error += SSR / run.0.weekly_infected.len() as f32;
    // }
    // // println!("average_error {}", average_error / N as f32);
    // average_error / N as f32
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

    let mut rng = rand::thread_rng();
    // create n=INDIVIDUALS individuals
    for _ in 0..INDIVIDUALS {
        // create the individual
        let x = rng.gen_range(0.0..=1.0_f32).to_string(); // spread chance
        let y = rng.gen_range(0.0..=1.0_f32).to_string(); // recovery chance
                                                          // let y = rng.gen_range(0.14..=0.3_f32).to_string(); // recovery chance beween 3 and 7 days
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
    let mut rng = rand::thread_rng();

    let mut children: Vec<String> = Vec::new();

    let twenty_perc = INDIVIDUALS as f32 * 0.2;
    for i in 0..(twenty_perc as usize) {
        children.push(population[i].clone());
    }

    let children_num = INDIVIDUALS as f32 - twenty_perc;

    for _ in 0..(children_num as usize) {
        // select two random individuals
        let idx_one = rng.gen_range(0..population.len());
        let parent_one = population[idx_one].clone();
        let mut idx_two = rng.gen_range(0..population.len());
        while idx_one == idx_two {
            idx_two = rng.gen_range(0..population.len());
        }
        let parent_two = population[idx_two].clone();

        let parent_one: Vec<&str> = parent_one.split(';').collect();
        let one_spread = parent_one[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let one_recovery = parent_one[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");

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

        if one_spread <= two_spread {
            min = one_spread;
            max = two_spread;
        } else {
            min = two_spread;
            max = one_spread;
        }
        let mut range = max - min;
        let mut p_min = 0.;
        let mut p_max = 1.0;
        if min - (range * alpha) > 0. {
            p_min = min - (range * alpha);
        }
        if max + (range * alpha) < 1.0 {
            p_max = max + (range * alpha);
        }
        let new_spread = rng.gen_range(p_min..=p_max);

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
        let new_recovery = rng.gen_range(p_min..=p_max);

        let new_individual = format!("{};{}", new_spread, new_recovery);
        children.push(new_individual);
    }

    *population = children;

    // let len = population.len();
    // while additional_individuals > 0 {
    //     // select two random individuals
    //     let idx_one = rng.gen_range(0..len);
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

    let mut rng = rand::thread_rng();
    // mutate one random parameter
    // randomly increase or decrease spread orrecovery
    if rng.gen_bool(MUTATION_RATE) {

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

        new_spread = rng.gen_range(min..=max);

        // mutate recovery
        let mut new_recovery = one_recovery
        .parse::<f32>()
        .expect("Unable to parse str to f32!");

        let alpha = 0.1;

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

        new_recovery = rng.gen_range(min..=max);
       
        new_ind = format!("{};{}", new_spread, new_recovery);

        *individual = new_ind;
    }
}

// fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
//     // Sort the array using the RT
//     // computed_ind.sort_by(|s1, s2| s1.0.rt.partial_cmp(&s2.0.rt).unwrap_or(Equal));

//     // println!("Sorted RT: -------------------------------");
//     // for i in 0..computed_ind.len() {
//     //     print!("{}\t", computed_ind[i].0.rt);
//     // }
//     // println!("\n-------------------------------");

//     // Get the median of the array
//     // let index = (computed_ind.len() + 1) / 2;
//     // let mut median = computed_ind[index - 1].0.rt;
//     // median = median * ( 1. - (30.computed_ind[index - 1].1.step as f32) / 30.);

//     // 1. - (DESIRED_RT - median).abs() / (DESIRED_RT.powf(2.) + median.powf(2.)).sqrt()

//     let mut sum_rt = 0.;
//     let mut rt_norm;
//     for ind in &*computed_ind {
//         rt_norm = ind.0.rt * (1. - (30. - ind.1.step as f32) / 30.);
//         sum_rt += rt_norm;
//     }

//     let mut avg = sum_rt / computed_ind.len() as f32;
//     avg = (avg * 1000.).round() / 1000.;

//     let fitness = 1. - (DESIRED_RT - avg).abs() / (DESIRED_RT.powf(2.) + avg.powf(2.)).sqrt();
//     // println!("Fitness: RT is {:?} - fitness is {}", avg, fitness);

//     // let to_write = format!(
//     //     "RT {} - Fitness {} - Spread {} - Recovery {}",
//     //     avg, fitness, computed_ind[0].0.spread, computed_ind[0].0.recovery
//     // );

//     // let mut file = OpenOptions::new()
//     //     .read(true)
//     //     .append(true)
//     //     .write(true)
//     //     .create(true)
//     //     .open("results.txt")
//     //     .unwrap();

//     // writeln!(file, "{:#?}", to_write).expect("Unable to write file.");

//     fitness
// }

// fn cmp(fitness1: &f32, fitness2: &f32) -> bool {
//     *fitness1 > *fitness2
// }
