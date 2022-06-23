use crate::model::node::{NetNode, NodeStatus};
use crate::{INITIAL_INFECTED_PROB, INIT_EDGES};
use krabmaga::engine::fields::network::Network;
use krabmaga::engine::fields::{field::Field, field_2d::Field2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;

pub struct EpidemicNetworkState {
    pub step: u64,
    pub field1: Field2D<NetNode>,
    pub network: Network<NetNode, String>,
    pub discretization: f32,
    pub toroidal: bool,
    pub dim: (f32, f32),
    pub num_nodes: u32,
}

impl EpidemicNetworkState {
    pub fn new(dim: (f32, f32), num_nodes: u32, d: f32, t: bool) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(dim.0, dim.1, d, t),
            network: Network::new(false),
            discretization: d,
            toroidal: t,
            dim,
            num_nodes,
        }
    }
}

impl State for EpidemicNetworkState {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Field2D::new(self.dim.0, self.dim.1, self.discretization, self.toroidal);
        self.network = Network::new(false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut node_set = Vec::new();
        let mut rng = rand::thread_rng();
        self.reset();
        for node_id in 0..self.num_nodes {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let init_status: NodeStatus = if rng.gen_bool(INITIAL_INFECTED_PROB) || node_id == 0 {
                NodeStatus::Infected
            } else {
                NodeStatus::Susceptible
            };

            let node = NetNode::new(
                node_id,
                Real2D {
                    x: self.dim.0 * r1,
                    y: self.dim.1 * r2,
                },
                init_status,
            );
            self.field1.set_object_location(node, node.loc);
            self.network.add_node(node);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
            node_set.push(node);
        }
        self.network
            .preferential_attachment_BA(&node_set, INIT_EDGES);
    }

    fn update(&mut self, step: u64) {
        self.field1.lazy_update();
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

    fn after_step(&mut self, _schedule: &mut Schedule) {
        // let mut susceptible: usize = 0;
        // let mut infected: usize = 0;
        // let mut resistant: usize = 0;
        // let agents = schedule.get_all_events();

        // for n in agents {
        //     let agent = n.downcast_ref::<NetNode>().unwrap();
        //     match agent.status {
        //         NodeStatus::Susceptible => {
        //             susceptible += 1;
        //         }
        //         NodeStatus::Infected => {
        //             infected += 1;
        //         }
        //         NodeStatus::Resistant => {
        //             resistant += 1;
        //         }
        //     }
        // }
        // println!(
        //     "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
        //     susceptible,
        //     infected,
        //     resistant,
        //     susceptible + infected + resistant
        // );
    }
}
