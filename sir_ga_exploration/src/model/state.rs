use crate::model::node::{NetNode, NodeStatus};
use crate::{DISCRETIZATION, WIDTH, HEIGHT, NUM_NODES, TOROIDAL, INIT_EDGES};
use rust_ab::engine::fields::network::Network;
use rust_ab::engine::fields::{field::Field, field_2d::Field2D};
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::engine::location::Real2D;
use rust_ab::rand::Rng;
use rust_ab::rand;
use std::any::Any;

pub struct EpidemicNetworkState {
    pub step: u64,
    pub field1: Field2D<NetNode>,
    pub network: Network<NetNode, String>,
    pub positions: Vec<u32>,
    pub fitness: f32,
}

impl EpidemicNetworkState {
    pub fn new(positions: Vec<u32>) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
            network: Network::new(false),
            positions: positions,
            fitness: 0.,
        }
    }
}

impl State for EpidemicNetworkState {
    fn init(&mut self, schedule: &mut Schedule) {
        let my_seed: u64 = 0;
        let mut node_set = Vec::new();
        self.network = Network::new(false);

        let mut rng = rand::thread_rng();
        let mut init_status: NodeStatus = NodeStatus::Susceptible;
        
        for node_id in 0..NUM_NODES {
            
            match self.positions[node_id as usize] {
                0 => init_status = NodeStatus::Susceptible,
                1 => init_status = NodeStatus::Immune,
                2 => init_status = NodeStatus::Infected,
                _ => (),
            }

            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

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
        self.network.preferential_attachment_BA_with_seed(&node_set, INIT_EDGES, my_seed);
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
    //     let mut susceptible: usize = 0;
    //     let mut infected: usize = 0;
    //     let mut resistant: usize = 0;
    //     let agents = schedule.get_all_events();

    //     for n in agents {
    //         let agent = n.downcast_ref::<NetNode>().unwrap();
    //         match agent.status {
    //             NodeStatus::Susceptible => {
    //                 susceptible += 1;
    //             }
    //             NodeStatus::Infected => {
    //                 infected += 1;
    //             }
    //             NodeStatus::Resistant => {
    //                 resistant += 1;
    //             }
    //         }
    //     }
    //     println!(
    //         "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
    //         susceptible,
    //         infected,
    //         resistant,
    //         susceptible + infected + resistant
    //     );
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

        if infected == 0 {
            // println!("No more infected nodes at step {}, exiting.", schedule.step);
            return true;
        }
        false
    }
}
