use rand::distributions::weighted::WeightedIndex;
use rand::seq::SliceRandom;

use rust_ab::{
    *,
    engine::{schedule::Schedule, location::Real2D, state::State, fields::network::Network},
    rand::Rng,
};

use model::state::EpidemicNetworkState;
use model::node::NodeStatus;
use model::node::NetNode;
mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;

///Initial infected nodes
pub static INITIAL_INFECTED_PROB: f64 = 0.01;
pub static INIT_EDGES: usize = 2;
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
pub static RECOVERY_CHANCE: f64 = 0.30;
pub static GAIN_RESISTENCE_CHANCE: f64 = 0.20;

pub static NUM_NODES: u32 = 100;

pub const MUTATION_RATE: f64 = 0.05;
pub const DESIRED_FITNESS: f32 = 0.90;
pub const MAX_GENERATION: u32 = 10;
pub const POPULATION: u32 = 10;

pub const WIDTH: f32 = 100.;
pub const HEIGHT: f32 = 100.;
pub const STEP: u64 = 100;

fn main() {
    
    let result = explore_ga!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        EpidemicNetworkState,
        DESIRED_FITNESS,
        MAX_GENERATION,
        STEP,
        ComputationMode::Sequential,
        parameters{
            //network: Network<NetNode, String>
            // positions: [u32]
        }
    );

    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name ="explore_result".to_string();
        let _res = write_csv(&name, &result);
    }
}

// function that initialize the populatin
fn init_population() -> Vec<EpidemicNetworkState> {
    // create an array of EpidemicNetworkState
    let mut population = Vec::new();
    
    let mut network: Network<NetNode, String> = Network::new(false);
    let mut node_set = Vec::new();
    let mut rng = rand::thread_rng();

    for node_id in 0..NUM_NODES {
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        let node = NetNode::new(
            node_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGHT * r2,
            },
            NodeStatus::Susceptible,
        );
        node_set.push(node);
    }
    
    network.preferential_attachment_BA(node_set, INIT_EDGES);
    println!("- {}", network.nodes2id.borrow().len());
    println!("- {}", network.id2nodes.borrow().len());
    println!("- {}", network.rid2nodes.borrow().len());

    println!("---{}", network.edges.borrow().len());
    println!("---{}", network.redges.borrow().len());

    // create n=POPULATION individuals
    for _ in 0..POPULATION {
        let mut rng = rand::thread_rng();        
        let mut network2: Network<NetNode, String> = Network::new(false);
    
        // create the individual
        let mut state = EpidemicNetworkState::new();
        network2.edges = network.edges.clone();
        network2.redges = network.redges.clone();
        network2.nodes2id = network.nodes2id.clone();
        network2.id2nodes = network.id2nodes.clone();
        network2.rid2nodes = network.rid2nodes.clone();
        network2.direct = network.direct.clone();
        state.network = network2;
        println!("{:?}", state.network.edges.borrow().len());
        population.push(state);
    }

    // return the array of individuals, i.e. the population
    population
}

fn selection(population: &mut Vec<EpidemicNetworkState>) {
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

fn crossover(population: &mut Vec<EpidemicNetworkState>) {
    let mut rng = rand::thread_rng();

    let additional_individuals = POPULATION as usize - population.len() ;
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
        let new_individual = EpidemicNetworkState::new();

        // add the new individual to the population
        population.push(new_individual);
    }
}

fn mutation(state: &mut EpidemicNetworkState) {
    let mut rng = rand::thread_rng();

    // mutate one random parameter with assigning random value
    if rng.gen_bool(MUTATION_RATE) {
        // let switch = rng.gen_range(0..3);
        // match switch {
        //     0 => ,
        //     1 => ,
        //     2 => ,
        //     _ => panic!("Invalid mutation switch"),
        // }
    }
}

fn fitness(state: &mut EpidemicNetworkState, schedule: Schedule) -> f32 {

    let mut susceptible: usize = 0;
    let mut infected: usize = 0;
    let mut resistent: usize = 0;

    let agents = schedule.get_all_events();

    for n in agents {
        let agent = n.downcast_ref::<NetNode>().unwrap();
        match agent.status {
            NodeStatus::Susceptible => {
                susceptible += 1;
            }
            NodeStatus::Infected => {
                infected += 1;
            }
            NodeStatus::Resistent => {
                resistent += 1;
            }
        }
    }

    let fitness = infected as f32 / NUM_NODES as f32 ;

    state.fitness = fitness;
    fitness
}
