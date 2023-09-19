use crate::model::node::{NetNode, NodeStatus};
use crate::{INIT_EDGES, NUM_NODES, STEP};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::network::Network;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::fmt;
use krabmaga::rand;
use rand::prelude::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

pub const MY_SEED: u64 = 0;

pub struct EpidemicNetworkState {
    pub step: u64,
    pub network: Network<NetNode, String>,
    pub recovery: f32,
    pub spread: f32,
    pub initial_infected: usize,
    pub rt: f32,
    pub infected_nodes: Vec<u32>, // each position of the array corresponds to one node
    pub daily_infected: Vec<u32>, // each position corresponds to the newly infected nodes
    pub old_infected: u32,
    pub weekly_infected: Vec<f32>,
    pub rng: Arc<Mutex<StdRng>>,
    pub day: u64,
    pub spread2: f32,
}

impl EpidemicNetworkState {
    #[allow(dead_code)]
    pub fn new(
        spread: f32,
        recovery: f32,
        spread2: f32,
        day: u64,
        initial_infected: usize,
    ) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            network: Network::new(false),
            rng: Arc::new(Mutex::new(StdRng::seed_from_u64(MY_SEED))),
            spread,           // virus spread chanceMY_SEED
            recovery,         // node recovery chance
            spread2,          // virus spread chance second period
            day,              // day when spread2 is applied
            initial_infected, // id of the initial infected node
            rt: 0.,           // transmission rate
            infected_nodes: vec![0; NUM_NODES as usize],
            old_infected: 0,
            daily_infected: vec![0; STEP as usize],
            weekly_infected: vec![0.; STEP as usize],
        }
    }

    // GA required new function to convert the string into parameters
    #[allow(dead_code)]
    pub fn new_with_parameters(r: usize, parameters: &str) -> EpidemicNetworkState {
        let parameters_ind: Vec<&str> = parameters.split(';').collect();
        let spread = parameters_ind[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let recovery = parameters_ind[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let spread2 = parameters_ind[2]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let day = parameters_ind[3]
            .parse::<u64>()
            .expect("Unable to parse str to usize!");
        EpidemicNetworkState::new(spread, recovery, spread2, day, r)
    }
}

impl fmt::Display for EpidemicNetworkState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rt {:.4} - Step {}", self.rt, self.step)
    }
}

impl State for EpidemicNetworkState {
    fn init(&mut self, schedule: &mut Schedule) {
        let mut node_set = Vec::new();
        self.network = Network::new(false);
        self.rt = 0.;
        // build a support array having the NodeStatus configuration
        let mut positions = vec![0; NUM_NODES as usize];

        let infected =
            StdRng::seed_from_u64(self.initial_infected as u64).gen_range(0..NUM_NODES) as usize;
        positions[infected] = 1;

        // positions[self.initial_infected] = 1;

        // generates nodes
        for node_id in 0..NUM_NODES {
            let init_status = match positions[node_id as usize] {
                0 => NodeStatus::Susceptible,
                1 => NodeStatus::Infected,
                _ => panic!("Invalid code for NodeStatus!"),
            };

            let node = NetNode::new(node_id, init_status);
            self.network.add_node(node);

            let _ = schedule.schedule_repeating(Box::new(node), 0.0, node_id as i32);

            node_set.push(node);
        }

        self.network.update();
        self.network
            .preferential_attachment_BA_with_seed(&node_set, INIT_EDGES, MY_SEED);
    }

    fn update(&mut self, step: u64) {
        self.network.update();
        self.step = step;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
    // if self.network.nodes2id.borrow().len() != 40 {
    //     println!("Init Len {} {}", self.network.nodes2id.borrow().len(), self.network.id2nodes.borrow().len());
    // }
    // if self.step == 0 {
    //     println!("Spread {} - Recovery {}", self.spread, self.recovery);
    // }
    // // if self.step == 0 {
    //     let mut susceptible: usize = 0;
    //     let mut infected: usize = 0;
    //     let mut resistant: usize = 0;
    // let agents = schedule.get_all_events();

    // for n in agents {
    //     let agent = n.downcast_ref::<NetNode>().unwrap();
    //     self.scheduled.push(format!(" {} {} ", self.step.to_string(), agent.id.to_string()));
    // }
    //     println!(
    //         "BEFORE Day {} Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
    //         self.step,
    //         susceptible,
    //         infected,
    //         resistant,
    //         susceptible + infected + resistant
    //     );
    //     // println!("RT is {}", self.rt);
    // // }
    // }
    fn reset(&mut self) {
        self.step = 0;
    }

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
        let infected_nodes = &self.infected_nodes;
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
            self.rt = value as f32 / counter as f32;
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
                media_mobile += self.daily_infected[((i as i32) - (j)) as usize] as f32;
            }
            self.weekly_infected[i - 3] = media_mobile / 7.0;
        }

        false
    }
}
