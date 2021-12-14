use crate::model::node::{NetNode, NodeStatus};
use crate::{INITIAL_INFECTED_PROB, NUM_NODES, DISCRETIZATION, TOROIDAL, WIDTH, HEIGHT};
use rust_ab::engine::fields::network::{Network, Edge, EdgeOptions};
use rust_ab::engine::fields::{field::Field, field_2d::Field2D};
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
    pub node_set: Vec<NetNode>,
    pub edge_set: Vec<Vec<Edge<String>>>,
    pub fitness: f32,
}

impl EpidemicNetworkState {

    pub fn new() -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
            network: Network::new(false),
            positions: Vec::with_capacity(NUM_NODES as usize),
            node_set: Vec::new(),
            edge_set: Vec::new(),
            fitness: 0.,
        }
    }

    pub fn set_network(&mut self, node_set: &mut Vec<NetNode>, edge_set: &mut Vec<Vec<Edge<String>>>) {

        for i in 0..NUM_NODES{
            self.network.add_node(node_set[i as usize]);
        }
        self.network.update();

        for i in 0..NUM_NODES{
            for j in 0..edge_set[i as usize].len(){
                let edge = &edge_set[i as usize][j];
                let node_u = self.network.get_object(edge.u).unwrap();
                let node_v = self.network.get_object(edge.v).unwrap();
                self.network.add_edge(node_u, node_v, EdgeOptions::Simple);
            }
        }

        self.network.update();

        self.node_set = node_set.to_vec();
        self.edge_set = edge_set.to_vec();
    }

    pub fn get_network(&self) 
        -> (Vec<NetNode>, Vec<Vec<Edge<String>>>){
            (self.node_set.clone(), self.edge_set.clone())
        }

}

impl State for EpidemicNetworkState {

    fn init(&mut self, schedule: &mut Schedule) {

        self.positions.clear();

        let mut rng = rand::thread_rng();
        
        for node_id in 0..NUM_NODES{
            let mut node = match self.network.get_object(node_id){
                Some(node) => node,
                None => panic!("Node with id {} not found!", node_id),
            };
 
            if rng.gen_bool(INITIAL_INFECTED_PROB) {
                self.positions.push(1);
            } else {
                self.positions.push(0);
            };
 
            match self.positions[node_id as usize] {
                0 => node.status = NodeStatus::Susceptible,
                1 => node.status = NodeStatus::Infected,
                _ => ()
            }
            self.network.update_node(node);

            self.field1.set_object_location(node, node.loc);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
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

    // fn before_step(&mut self, schedule: &mut Schedule) {
    //     let mut susceptible: usize = 0;
    //     let mut infected: usize = 0;
    //     let mut resistent: usize = 0;
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
    //             NodeStatus::Resistent => {
    //                 resistent += 1;
    //             }
    //         }
    //     }
    //     println!(
    //         "Susceptible: {:?} Infected: {:?} Resistant: {:?} Tot: {:?}",
    //         susceptible,
    //         infected,
    //         resistent,
    //         susceptible + infected + resistent
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

        if infected == 0{
            // println!("No more infected nodes at step {}, exiting.", schedule.step);
            return true;
        }
        false
    }
}
