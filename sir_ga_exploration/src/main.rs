use rand::distributions::weighted::WeightedIndex;

use rust_ab::{
    engine::{schedule::Schedule, state::State},
    rand::Rng,
    *,
};

use model::node::NetNode;
use model::node::NodeStatus;
use model::state::EpidemicNetworkState;
mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;

///Initial infected nodes
pub static INIT_EDGES: usize = 2;
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
pub static RECOVERY_CHANCE: f64 = 0.3;
pub static GAIN_RESISTANCE_CHANCE: f64 = 0.2;

pub static INITIAL_IMMUNE: f32 = 0.3;
pub static INITIAL_INFECTED: f32 = 0.1;
pub const NUM_NODES: u32 = 100;

pub const MUTATION_RATE: f64 = 0.05;
pub const DESIRED_FITNESS: f32 = 1.;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 500;

pub const WIDTH: f32 = 150.;
pub const HEIGHT: f32 = 150.;

pub const STEP: u64 = 500;

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
        1,
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

        // let mut positions = vec![0 as u8; NUM_NODES as usize];

        let mut positions = String::with_capacity(NUM_NODES as usize);
        for _ in 0..NUM_NODES{
            positions.push('0');
        }
    
        let mut immune_counter = 0;
        while immune_counter != (INITIAL_IMMUNE * NUM_NODES as f32) as u32 {
            
            let node_id = rng.gen_range(0..NUM_NODES) as usize;
            
            if positions.chars().nth(node_id).unwrap() == '0' {
                positions.replace_range(node_id..node_id+1,"1");
                immune_counter += 1;
            }
        }
        
        let mut infected_counter = 0;
        while infected_counter != (INITIAL_INFECTED * NUM_NODES as f32) as u32 {
            
            let node_id = rng.gen_range(0..NUM_NODES) as usize;

            if positions.chars().nth(node_id).unwrap() == '0' {
                positions.replace_range(node_id..node_id+1,"2");
                infected_counter += 1;
            }
        }

        population.push(positions.clone());
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
        let mut parent_one = population[idx_one].clone();
        let mut parent_two = population[idx_two].clone();

        let len = parent_one.len() / 2;

        parent_one.truncate(len);

        let positions_one = parent_one;
        let positions_two = parent_two.split_off(len);

        let new_individual = format!("{}{}", positions_one, positions_two);

        // create a new individual
        
        population.push(new_individual);
    }
}

fn mutation(individual: &mut String) {
    let mut rng = rand::thread_rng();

    // mutate one random parameter with assigning random value
    if rng.gen_bool(MUTATION_RATE) {
        let to_change = rng.gen_range(0..NUM_NODES as usize) as usize;
        if individual.chars().nth(to_change).unwrap() == '0' {
            individual.replace_range(to_change..to_change+1,"1");
        } else {
            individual.replace_range(to_change..to_change+1,"0");
        }
    }
}

fn fitness(computed_ind: &mut Vec<(EpidemicNetworkState, Schedule)>) -> f32 {
    let schedule = &computed_ind[0].1;

    let mut _susceptible: usize = 0;
    let mut _infected: usize = 0;
    let mut resistant: usize = 0;
    let mut _immune: usize = 0;

    let agents = schedule.get_all_events();

    for n in agents {
        let agent = n.downcast_ref::<NetNode>().unwrap();
        match agent.status {
            NodeStatus::Susceptible => {
                _susceptible += 1;
            }
            NodeStatus::Infected => {
                _infected += 1;
            }
            NodeStatus::Resistant => {
                resistant += 1;
            }
            NodeStatus::Immune => {
                _immune += 1;
            }
        }
    }

    // println!(
    //     "Susceptible: {:?} Infected: {:?} Resistant: {:?} Immune: {:?} Tot: {:?}",
    //     susceptible,
    //     infected,
    //     resistant,
    //     immune,
    //     susceptible + infected + resistant + immune
    // );
    
    1. - (resistant as f32 / NUM_NODES as f32)
}