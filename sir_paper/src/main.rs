use rand::distributions::weighted::WeightedIndex;

use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use model::state::EpidemicNetworkState;
// use std::cmp::Ordering::Equal;
// use std::io::Write;
mod model;

// generic model parameters
pub static INIT_EDGES: usize = 3;
pub const NUM_NODES: u32 = 5_000;
pub static DESIRED_RT: f32 = 2.;
// pub static INITIAL_INFECTED: f32 = 0.01;

// GA specific parameters
pub const MUTATION_RATE: f64 = 0.1;
pub const DESIRED_FITNESS: f32 = 2.;
pub const MAX_GENERATION: u32 = 2;
pub const INDIVIDUALS: u32 = 100;
pub const REPETITION: u32 = 1;

lazy_static! {
    pub static ref data: Vec<f32> = {
        let mut rdr = Reader::from_path("data/data_perc.csv").unwrap();
        
        let mut x: Vec<f32> = Vec::new();
        
        for result in rdr.records() {
            let record = result.unwrap();
            let mut i = 0;           
            let y : f32 = record[i].parse().unwrap();
            x.push(y);
            i += 1;
        }
        x
     };
}

// compute the rt at 30 steps (30 days)
pub const STEP: u64 = 66;

fn main() {
    // let epidemic_network = EpidemicNetworkState::new_with_parameters("0.15;0.02");
    // simulate!(STEP, epidemic_network, 1, Info::Verbose);
    
    
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
    // create an array of EpidemicNetworkState
    let mut population = Vec::new();

    // create n=INDIVIDUALS individuals
    for _ in 0..INDIVIDUALS {
        // create the individual
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.35..=0.45_f32).to_string(); // spread chance
        let y = rng.gen_range(0.05..=0.2_f32).to_string(); // recovery chance
        // let y = rng.gen_range(0.14..=0.3_f32).to_string(); // recovery chance

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
    // a = xa, ya || b = xb, yb
    // child will be (xa, yb) or (xb, ya)

    let mut parents = population.clone();

    let mut additional_individuals = INDIVIDUALS as usize - population.len(); // N/2

    while additional_individuals > 0 {
        // select two random individuals
        let idx_one = rng.gen_range(0..parents.len());
        let parent_one = parents[idx_one].clone();
        parents.remove(idx_one);

        if additional_individuals == 1 {
            population.push(parent_one);
            additional_individuals -= 1;
            continue;
        }

        let idx_two = rng.gen_range(0..parents.len());
        let parent_two = parents[idx_two].clone();
        parents.remove(idx_two);

        let parent_one: Vec<&str> = parent_one.split(';').collect();
        let one_spread = parent_one[0];
        let one_recovery = parent_one[1];

        let parent_two: Vec<&str> = parent_two.split(';').collect();
        let two_spread = parent_two[0];
        let two_recovery = parent_two[1];

        let new_individual_one = format!("{};{}", one_spread, two_recovery);
        let new_individual_two = format!("{};{}", two_spread, one_recovery);

        population.push(new_individual_one);
        population.push(new_individual_two);

        additional_individuals -= 2;
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
            let mut new_spread = one_spread
                .parse::<f32>()
                .expect("Unable to parse str to f32!");
            if rng.gen_bool(0.5) {
                if new_spread + x < 1.0 {
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
            let mut new_recovery = one_recovery
                .parse::<f32>()
                .expect("Unable to parse str to f32!");

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
    
    let mut error = 0.;
    // per ogni simulazione
    for i in 0..computed_ind.len() {

        if computed_ind[i].1.step < 36 {
            return f32::MAX;
        }

        // cicliamo per normalizzare il weekly_infected
        for j in 0..computed_ind[i].0.weekly_infected.len(){
            computed_ind[i].0.weekly_infected[j] /= NUM_NODES as f32;
        }
        println!("Fitness weekly infected {:?}", computed_ind[i].0.weekly_infected);

        // calcolo l'errore rispetto agli observed results usando la formula
        // sommatoria da i=0 a n=30 di (s[i] - o[i])^2
        // dove s[i] è la media settimanale del giorno i calcolato dalla simulazione
        // e o[i] è la media settimanale del giorno i osservato (contenuto nel file csv)
        let mut ind_error = 0.;
        for k in 0..30 {
            ind_error += (computed_ind[i].0.weekly_infected[k] - data[k]).powf(2.);
        }
        println!("Fitness ind {} ind_error {:?}", i, ind_error);
        error += ind_error;
    }
    
    println!("Fitness average is {}", error / computed_ind.len() as f32);
    println!("");
    println!("");
    println!("");
    error / computed_ind.len() as f32
}

/*
fn old_fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
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
    // median = median * ( 1. - (30. - computed_ind[index - 1].1.step as f32) / 30.);

    // 1. - (DESIRED_RT - median).abs() / (DESIRED_RT.powf(2.) + median.powf(2.)).sqrt()

    let mut sum_rt = 0.;
    let mut rt_norm;
    for ind in &*computed_ind {
        rt_norm = ind.0.rt * (1. - (30. - ind.1.step as f32) / 30.);
        sum_rt += rt_norm;
    }

    let mut avg = sum_rt / computed_ind.len() as f32;
    avg = (avg * 1000.).round() / 1000.;

    let fitness = 1. - (DESIRED_RT - avg).abs() / (DESIRED_RT.powf(2.) + avg.powf(2.)).sqrt();
    // println!("Fitness: RT is {:?} - fitness is {}", avg, fitness);

    // let to_write = format!(
    //     "RT {} - Fitness {} - Spread {} - Recovery {}",
    //     avg, fitness, computed_ind[0].0.spread, computed_ind[0].0.recovery
    // );

    // let mut file = OpenOptions::new()
    //     .read(true)
    //     .append(true)
    //     .write(true)
    //     .create(true)
    //     .open("results.txt")
    //     .unwrap();

    // writeln!(file, "{:#?}", to_write).expect("Unable to write file.");

    fitness
}
 */