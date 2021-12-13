use crate::model::node::{NetNode, NodeStatus};
use crate::{INITIAL_INFECTED_PROB, INIT_EDGES, NUM_NODES, DISCRETIZATION, TOROIDAL, WIDTH, HEIGHT};
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
    pub positions: Vec<u32>,
    pub fitness: f32,
}

impl EpidemicNetworkState {

    pub fn new() -> EpidemicNetworkState 
    {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
            network: Network::new(false),
            positions: Vec::new(),
            fitness: 0.,
        }
    }

    // pub fn new(network : &Network<NetNode, String>) -> EpidemicNetworkState {
    //     EpidemicNetworkState {
    //         step: 0,
    //         field1: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
    //         network,
    //         positions: Vec::new(),
    //         fitness: 0.,
    //     }
    // }
}

impl State for EpidemicNetworkState {

    fn init(&mut self, schedule: &mut Schedule) {
        for node_id in 0..NUM_NODES {
            
            for vec in self.field1.rbags.borrow_mut().iter_mut() {
                for node in vec.iter_mut() {
                    if node.status == NodeStatus::Infected {
                        self.positions.push(1);
                    } else {
                        self.positions.push(0);
                    }                
                }
            }
            
        }
        
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

    fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
        let mut infected: usize = 0;
        let agents = schedule.get_all_events();

        for n in agents {
            let agent = n.downcast_ref::<NetNode>().unwrap();
            if agent.status == NodeStatus::Infected {
                infected += 1;
            }
        }

        if infected == 0{
            println!("No more infected nodes at step {}, exiting.", schedule.step);
            return true;
        }
        false
    }
}
