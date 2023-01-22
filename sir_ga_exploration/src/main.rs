use krabmaga::*;

#[cfg(any(feature = "distributed_mpi"))]
use krabmaga::{engine::schedule::Schedule, engine::state::State, rand::Rng};
use rand::prelude::*;

#[cfg(any(feature = "distributed_mpi"))]
use {crate::model::state::EpidemicNetworkState, std::cmp::Ordering::Equal};
mod model;

// generic model parameters
pub static INIT_EDGES: usize = 1;
pub const NUM_NODES: u32 = 1_000;

// GA specific parameters
lazy_static! {
    static ref MUTATION_RATE: Mutex<f64> = Mutex::new(0.8);
}
pub const DESIRED_FITNESS: f32 = 0.;
pub const MAX_GENERATION: u32 = 2_000;
pub const INDIVIDUALS: u32 = 100;
pub const REPETITIONS: u32 = 20;

lazy_static! {
    pub static ref DATA: Vec<f32> = {
        let mut rdr = Reader::from_path("data/data.csv").unwrap();

        let mut x: Vec<f32> = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            let y: f32 = record[0].parse().unwrap();
            x.push(y);
        }
        x
    };
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(0));
}

pub const STEP: u64 = 51; // 51 - 37
pub const DAY: usize = 45; // 45 - 31

#[cfg(not(any(feature = "distributed_mpi")))]
fn main() {
    println!("No bayesian feature enabled");
}

#[cfg(any(feature = "distributed_mpi"))]
fn main() {
    let result = explore_ga_distributed_mpi!(
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
        REPETITIONS,
    );
    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name = "explore_result".to_string();
        let _res = write_csv(&name, &result);
    }
}
#[cfg(any(feature = "distributed_mpi"))]
fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    let mut avg_results: Vec<f32> = vec![0.0; DAY];

    for j in 0..DAY {
        for i in 0..computed_ind.len() {
            avg_results[j] += computed_ind[i].0.weekly_infected[j] / NUM_NODES as f32;
        }
        avg_results[j] /= computed_ind.len() as f32;
    }

    let mut ind_error = 0.;
    let mut sum = 0.;
    for k in 0..DAY {
        let weight = (k + 1) as f32;
        ind_error += weight * (DATA[k] - avg_results[k]).abs();
        sum += weight * DATA[k];
    }
    ind_error = ind_error / sum;
    ind_error
}

// we want to minimize the fitness, therefore the comparison
// return true, meaning that fitness1 is better than fitness2,
// if fitness1 is lower than fitness 2
#[cfg(any(feature = "distributed_mpi"))]
fn cmp(fitness1: &f32, fitness2: &f32) -> bool {
    *fitness1 < *fitness2
}

// function that initialize the populatin
#[cfg(any(feature = "distributed_mpi"))]
fn init_population() -> Vec<String> {
    // create an array of EpidemicNetworkState
    let mut population = Vec::new();
    let mut rng = RNG.lock().unwrap();
    // create n=INDIVIDUALS individuals
    for _ in 0..INDIVIDUALS {
        // create the individual
        let x = rng.gen_range(0.0..=1.0_f32).to_string(); // spread chance
        let y = rng.gen_range(0.0..=1.0_f32).to_string(); // recovery chance
        let x2 = rng.gen_range(0.0..=1.0_f32).to_string(); // recovery chance
        let day = rng.gen_range(0..=DAY).to_string(); // recovery chance
        population.push(format!("{};{};{};{}", x, y, x2, day));
    }

    // return the array of individuals, i.e. the population (only the parameters)
    population
}

#[cfg(any(feature = "distributed_mpi"))]
fn selection(population_fitness: &mut Vec<(String, f32)>) {
    let mut min_fitness = 1.;
    for individual_fitness in population_fitness.iter_mut() {
        if individual_fitness.1 < min_fitness {
            min_fitness = individual_fitness.1;
        }
    }

    if min_fitness < 0.25 {
        *MUTATION_RATE.lock().unwrap() = 0.2;
    } else if min_fitness < 0.5 {
        *MUTATION_RATE.lock().unwrap() = 0.4;
    } else if min_fitness < 0.75 {
        *MUTATION_RATE.lock().unwrap() = 0.6;
    }

    // sort the population based on the fitness
    population_fitness.sort_by(|s1, s2| s1.1.partial_cmp(&s2.1).unwrap_or(Equal));
}

#[cfg(any(feature = "distributed_mpi"))]
fn crossover(population: &mut Vec<String>) {
    let mut children: Vec<String> = Vec::new();

    let perc = INDIVIDUALS as f32 * 0.2;
    for i in 0..(perc as usize) {
        children.push(population[i].clone());
    }

    let children_num = INDIVIDUALS as f32 - perc;

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
        let one_spread2 = parent_one[2]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let one_day = parent_one[3]
            .parse::<u64>()
            .expect("Unable to parse str to f32!");

        // take the parameters of parent_two
        let parent_two: Vec<&str> = parent_two.split(';').collect();
        let two_spread = parent_two[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let two_recovery = parent_two[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let two_spread2 = parent_two[2]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let two_day = parent_two[3]
            .parse::<u64>()
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

        // new_spread2
        if one_spread2 <= two_spread2 {
            min = one_spread2;
            max = two_spread2;
        } else {
            min = two_spread2;
            max = one_spread2;
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
        let new_spread2 = RNG.lock().unwrap().gen_range(p_min..=p_max);

        // new_day
        let min_day;
        let max_day;
        if one_day <= two_day {
            min_day = one_day;
            max_day = two_day;
        } else {
            min_day = two_day;
            max_day = one_day;
        }
        let range_day: f32 = (max_day - min_day) as f32;
        p_min = 0.;
        p_max = DAY as f32;
        if min_day as f32 - (range_day * alpha).ceil() > 0. {
            p_min = min_day as f32 - (range_day * alpha);
        }
        if max_day as f32 + (range_day * alpha).ceil() < DAY as f32 {
            p_max = max_day as f32 + (range_day * alpha);
        }
        if p_min >= p_max {
            p_min = 0.;
            p_max = DAY as f32;
        }
        let new_day = RNG.lock().unwrap().gen_range(p_min..=p_max).ceil();

        let new_individual = format!(
            "{};{};{};{}",
            new_spread, new_recovery, new_spread2, new_day
        );
        children.push(new_individual);
    }

    *population = children;
}

#[cfg(any(feature = "distributed_mpi"))]
fn mutation(individual: &mut String) {
    let new_ind: String;
    let new_individual: Vec<&str> = individual.split(';').collect();
    let one_spread = new_individual[0];
    let one_recovery = new_individual[1];
    let one_spread2 = new_individual[2];
    let one_day = new_individual[3];

    // mutate one random parameter
    // randomly increase or decrease spread orrecovery
    if RNG.lock().unwrap().gen_bool(*MUTATION_RATE.lock().unwrap()) {
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

        let mut new_spread2 = one_spread2
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        min = if new_spread2 - alpha < 0. {
            0.
        } else {
            new_spread2 - alpha
        };
        max = if new_spread2 + alpha > 1. {
            1.
        } else {
            new_spread2 + alpha
        };
        if min >= max {
            min = 0.;
            max = 1.;
        }
        new_spread2 = RNG.lock().unwrap().gen_range(min..=max);

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

        let mut new_day = one_day.parse::<u64>().expect("Unable to parse str to u64!");
        let alpha: u64 = 1;
        // let mut min: u64 = if new_day - alpha < 0 {
        //     0
        // } else {
        //     new_day - alpha
        // };
        // this line is suggested by clippy
        let mut min: u64 = new_day - alpha;
        let mut max: u64 = if new_day + alpha > DAY as u64 {
            DAY as u64
        } else {
            new_day + alpha
        };
        if min >= max {
            min = 0;
            max = DAY as u64;
        }
        new_day = RNG.lock().unwrap().gen_range(min..=max);

        new_ind = format!(
            "{};{};{};{}",
            new_spread, new_recovery, new_spread2, new_day
        );
        *individual = new_ind;
    }
}
