use crate::model::node::{NetNode, NodeStatus};
use crate::{HEIGTH, INITIAL_INFECTED_PROB, INIT_EDGES, NUM_NODES, WIDTH};
use rust_ab::engine::fields::network::Network;
use rust_ab::engine::fields::{field::Field, field_2d::Field2D};
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;

pub struct EpidemicNetworkState {
    pub step: u64,
    pub field1: Field2D<NetNode>,
    pub network: Network<NetNode, String>,
    pub width: f32,
    pub height: f32,
    pub discretization: f32,
    pub toroidal: bool,
}

impl EpidemicNetworkState {
    pub fn new(w: f32, h: f32, d: f32, t: bool) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(w, h, d, t),
            network: Network::new(false),
            width: w,
            height: h,
            discretization: d,
            toroidal: t,
        }
    }
}

impl State for EpidemicNetworkState {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Field2D::new(self.width, self.height, self.discretization, self.toroidal);
        self.network = Network::new(false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut node_set = Vec::new();
        let mut rng = rand::thread_rng();
        for node_id in 0..NUM_NODES {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let init_status: NodeStatus = if rng.gen_bool(INITIAL_INFECTED_PROB) {
                NodeStatus::Infected
            } else {
                NodeStatus::Susceptible
            };

            let node = NetNode::new(
                node_id,
                Real2D {
                    x: WIDTH * r1,
                    y: HEIGTH * r2,
                },
                init_status,
            );
            self.field1.set_object_location(node, node.loc);
            self.network.add_node(node);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
            node_set.push(node);
        }
        self.network
            .preferential_attachment_BA(node_set, INIT_EDGES);
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

    fn after_step(&mut self, _schedule: &mut Schedule) {
        // let mut susceptible: usize = 0;
        // let mut infected: usize = 0;
        // let mut resistent: usize = 0;
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
        //         NodeStatus::Resistent => {
        //             resistent += 1;
        //         }
        //     }
        // }
        // println!(
        //     "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
        //     susceptible,
        //     infected,
        //     resistent,
        //     susceptible + infected + resistent
        // );
    }
}
