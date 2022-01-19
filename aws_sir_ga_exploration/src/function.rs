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
pub const DESIRED_FITNESS: f32 = 0.8;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 10;

pub const WIDTH: f32 = 150.;
pub const HEIGHT: f32 = 150.;

pub const STEP: u64 = 100;

fn dummy_main() {
    let result = explore_ga_aws!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        EpidemicNetworkState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        3,
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

fn fitness(state: &mut EpidemicNetworkState, schedule: Schedule) -> f32 {
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

    let fitness = 1. - (resistant as f32 / NUM_NODES as f32);

    state.fitness = fitness;
    fitness
}
use rust_ab::{
    lambda_runtime,
    aws_sdk_sqs,
    aws_config,
    tokio
};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = lambda_runtime::handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: lambda_runtime::Context) -> Result<(), lambda_runtime::Error> {

    // read the payload
    let my_population_params = event["individuals"].as_array().expect("Cannot parse individuals value from event!");

    // prepare the result json to send on the queue
    let mut results: String = format!("{{\n\t\"function\":[");
    
    for (index, ind) in my_population_params.iter().enumerate(){
        let individual = ind.as_str().expect("Cannot cast individual!").to_string();
        
        // initialize the state
        let mut individual_state = <EpidemicNetworkState>::new_with_parameters(&individual); // <$state>::new_with_parameters(&individual);
        let mut schedule: Schedule = Schedule::new();
        individual_state.init(&mut schedule);
        // compute the simulation
        for _ in 0..(STEP as usize) { // $step as usize
            let individual_state = individual_state.as_state_mut();
            schedule.step(individual_state);
            if individual_state.end_condition(&mut schedule) {
                break;
            }
        }

        // compute the fitness value
        let fitness = fitness(&mut individual_state, schedule); //$fitness(&mut individual_state, schedule);

        {
            results.push_str(&format!("\n\t{{\n\t\t\"Index\": {}, \n\t\t\"Fitness\": {}, \n\t\t\"Individual\": \"{}\"\n\t}},", index, fitness, individual).to_string());
        }
    }

    results.truncate(results.len()-1); // required to remove the last comma
    results.push_str(&format!("\n\t]\n}}").to_string());

    // send the result on the SQS queue
    send_on_sqs(results.to_string()).await;
    
    Ok(())
}

async fn send_on_sqs(results: String) -> Result<(), aws_sdk_sqs::Error> {
    // configuration of the aws client
	let region_provider = aws_config::meta::region::RegionProviderChain::default_provider();
	let config = aws_config::from_env().region(region_provider).load().await;

    // create the SQS client
	let client_sqs = aws_sdk_sqs::Client::new(&config);
    

    // get the queue_url of the queue
    let queue = client_sqs.get_queue_url().queue_name("rab_queue".to_string()).send().await?;
    let queue_url = queue.queue_url.expect("Cannot get the queue url!");

    let send_request = client_sqs
        .send_message()
        .queue_url(queue_url)
        .message_body(results)
        .send()
        .await?;

    Ok(())
}
// end of the lambda function
        