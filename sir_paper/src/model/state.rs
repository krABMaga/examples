use crate::model::node::{NetNode, NodeStatus};
use crate::{DISCRETIZATION, HEIGHT, INITIAL_INFECTED, INIT_EDGES, NUM_NODES, TOROIDAL, WIDTH};
use rust_ab::engine::fields::network::Network;
use rust_ab::engine::fields::{field::Field, field_2d::Field2D};
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;
use std::sync::{Arc, Mutex};

pub struct EpidemicNetworkState {
    pub step: u64,
    pub field1: Field2D<NetNode>,
    pub network: Network<NetNode, String>,
    pub infected_nodes: Arc<Mutex<Vec<u32>>>, // each position of the array corresponds to one node
    pub rt: f32,
    pub spread: f32,
    pub recovery: f32,
}

impl EpidemicNetworkState {
    pub fn new(spread: f32, recovery: f32) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
            network: Network::new(false),
            infected_nodes: Arc::new(Mutex::new(vec![0; NUM_NODES as usize])), // dimension is NUM_NODE
            rt: 0.,
            spread,
            recovery,
        }
    }

    // GA required new function to convert the string into parameters
    pub fn new_with_parameters(parameters: &str) -> EpidemicNetworkState {
        let parameters_ind: Vec<&str> = parameters.split(';').collect();
        let spread = parameters_ind[0]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        let recovery = parameters_ind[1]
            .parse::<f32>()
            .expect("Unable to parse str to f32!");
        EpidemicNetworkState::new(spread, recovery)
    }
}

impl State for EpidemicNetworkState {
    fn init(&mut self, schedule: &mut Schedule) {
        let mut rng = rand::thread_rng();
        let my_seed: u64 = 0;
        let mut node_set = Vec::new();
        self.network = Network::new(false);
        self.rt = 0.;
        let mut infected_counter = 0;

        // build a support array having the NodeStatus configuration
        let mut positions = vec![0; NUM_NODES as usize];

        // generate exactly INITIAL_INFECTED * NUM_NODES infected nodes
        while infected_counter != (INITIAL_INFECTED * NUM_NODES as f32) as u32 {
            let node_id = rng.gen_range(0..NUM_NODES) as usize;
            if positions[node_id] == 0 {
                positions[node_id] = 1;
                infected_counter += 1;
            }
        }

        // generates nodes
        for node_id in 0..NUM_NODES {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let init_status = match positions[node_id as usize] {
                0 => NodeStatus::Susceptible,
                1 => NodeStatus::Infected,
                _ => panic!("Ehm, that's not possible!"),
            };

            let node = NetNode::new(
                node_id,
                Real2D {
                    x: WIDTH * r1,
                    y: HEIGHT * r2,
                },
                init_status,
            );

            self.field1.set_object_location(node, node.loc);
            self.network.add_node(node);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
            node_set.push(node);
        }
        self.network.update();
        self.network
            .preferential_attachment_BA_with_seed(&node_set, INIT_EDGES, my_seed);
    }

    fn update(&mut self, step: u64) {
        self.field1.lazy_update();
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
    //     if self.step == 0 {
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
    //             "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
    //             susceptible,
    //             infected,
    //             resistant,
    //             susceptible + infected + resistant
    //         );
    //     }
    // }

    fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
        let mut infected: usize = 0;
        let agents = schedule.get_all_events();

        for n in agents {
            let agent = n.downcast_ref::<NetNode>().unwrap();
            if agent.status == NodeStatus::Infected {
                infected += 1;
            }
        }
        if self.step == 30 {
            // compute the RT after 30 days
            let mut counter = 0;
            let mut value = 0;
            let infected_nodes = self.infected_nodes.lock().unwrap();
            for i in 0..infected_nodes.len() {
                if infected_nodes[i] != 0 {
                    counter += 1;
                    value += infected_nodes[i];
                }
            }
            self.rt = (value as f32 / counter as f32) as f32;
        }
        if infected == 0 {
            if self.step < 30 {
                // println!("No more infected nodes at step {}, spread {}, recovery {} exiting.", schedule.step, self.spread, self.recovery);
            }
            return true;
        }
        false
    }
}
