use crate::model::node::{NetNode, NodeStatus};
use crate::{INIT_EDGES, NUM_NODES, STEP};
use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::network::Network;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::fmt;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;
use std::sync::{Arc, Mutex};

pub struct EpidemicNetworkState {
    pub step: u64,
    pub network: Network<NetNode, String>,
    pub recovery: f32,
    pub spread: f32,
    pub initial_infected: usize,
    pub rt: f32,
    pub infected_nodes: Arc<Mutex<Vec<u32>>>, // each position of the array corresponds to one node
    pub daily_infected: Vec<u32>, // each position corresponds to the newly infected nodes
    pub old_infected: u32,
    pub weekly_infected: Vec<f32>,
}

impl EpidemicNetworkState {
    pub fn new(spread: f32, recovery: f32, initial_infected: usize) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            network: Network::new(false),
            recovery,
            spread,
            initial_infected,
            rt: 0.,
            infected_nodes: Arc::new(Mutex::new(vec![0; NUM_NODES as usize])), // dimension is NUM_NODE
            old_infected: 0,
            daily_infected: vec![0; STEP as usize], // dimension is STEP
            weekly_infected: vec![0.; STEP as usize], // media settimanale giornaliera per 60 giorni
        }
    }

    // GA required new function to convert the string into parameters
    pub fn new_with_parameters(r: usize, parameters: &str) -> EpidemicNetworkState {
        let parameters_ind: Vec<&str> = parameters.split(';').collect();
        let spread = parameters_ind[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let recovery = parameters_ind[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");

        EpidemicNetworkState::new(spread, recovery, r)
    }
}

impl fmt::Display for EpidemicNetworkState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rt {:.4} - Step {}", self.rt, self.step)
    }
}

impl State for EpidemicNetworkState {
    fn init(&mut self, schedule: &mut Schedule) {

        let my_seed: u64 = 0;
        let mut node_set = Vec::new();
        self.network = Network::new(false);
        self.rt = 0.;

        // build a support array having the NodeStatus configuration
        let mut positions = vec![0; NUM_NODES as usize];

        // let mut rng = rand::thread_rng();
        // let mut infected_counter = 0;
        // generate exactly INITIAL_INFECTED * NUM_NODES infected nodes
        // while infected_counter != (INITIAL_INFECTED * NUM_NODES as f32) as u32 {
        //     let node_id = rng.gen_range(0..NUM_NODES) as usize;
        //     if positions[node_id] == 0 {
        //         positions[node_id] = 1;
        //         infected_counter += 1;
        //     }
        // }

        let node_id = self.initial_infected;
        positions[node_id] = 1;

        // generates nodes
        for node_id in 0..NUM_NODES {
            let init_status = match positions[node_id as usize] {
                0 => NodeStatus::Susceptible,
                1 => NodeStatus::Infected,
                _ => panic!("Invalid code for NodeStatus!"),
            };

            let node = NetNode::new(node_id, init_status);
            self.network.add_node(node);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
            node_set.push(node);
        }
        self.network.update();
        self.network
            .preferential_attachment_BA_with_seed(&node_set, INIT_EDGES, my_seed);
    }

    fn update(&mut self, step: u64) {
        self.network.update();
        self.step = step;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    // fn before_step(&mut self, schedule: &mut Schedule) {
    //     // if self.step == 0 {
    //     //     println!("Spread {} - Recovery {}", self.spread, self.recovery);
    //     // }
    //     // if self.step == 0 {
    //         let mut susceptible: usize = 0;
    //         let mut infected: usize = 0;
    //         let mut resistant: usize = 0;
    //         let agents = schedule.get_all_events();

    //         for n in agents {
    //             let agent = n.downcast_ref::<NetNode>().unwrap();
    //             match agent.status {
    //                 NodeStatus::Susceptible => {
    //                     susceptible += 1;
    //                 }
    //                 NodeStatus::Infected => {
    //                     infected += 1;
    //                 }
    //                 NodeStatus::Resistant => {
    //                     resistant += 1;
    //                 }
    //             }
    //         }
    //         println!(
    //             "BEFORE Day {} Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
    //             self.step,
    //             susceptible,
    //             infected,
    //             resistant,
    //             susceptible + infected + resistant
    //         );
    //         // println!("RT is {}", self.rt);
    //     // }
    // }

    fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
        // check if there are no more infected node
        let mut infected: usize = 0;
        let agents = schedule.get_all_events();
        for n in agents {
            let agent = n.downcast_ref::<NetNode>().unwrap();
            if agent.status == NodeStatus::Infected {
                infected += 1;
            }
        }
        if infected == 0 {
            return true;
        }

        // compute the rt
        let infected_nodes = self.infected_nodes.lock().unwrap();
        let mut counter = 0;
        let mut value = 0;
        for i in 3..infected_nodes.len() {
            if infected_nodes[i] != 0 {
                counter += 1;
                value += infected_nodes[i];
            }
        }
        if value == 0 {
            self.rt = 0.;
        } else {
            self.rt = (value as f32 / counter as f32) as f32;
        }
        
        // count the daily infection
        let mut newly_infected = 0;
        for i in 0..infected_nodes.len() {
            newly_infected += infected_nodes[i];
        } 
        
        // compute the daily weekly average of infection
        let output = newly_infected - self.old_infected;
        self.daily_infected[self.step as usize - 1] = output;
        self.old_infected = newly_infected;

        for i in 3..(self.daily_infected.len() - 3) {
            let mut media_mobile = 0.;
            for j in -3..=3 {
                media_mobile += self.daily_infected[((i as i32) - (j as i32)) as usize] as f32;
            }
            self.weekly_infected[i - 3] = media_mobile / 7.0; 
        }

        false
    }
}
