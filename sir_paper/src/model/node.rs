use std::{
    fmt,
    hash::{Hash, Hasher},
};

use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::{
    engine::{
        agent::Agent,
        location::{Location2D, Real2D},
    },
    rand::Rng,
};

use crate::model::state::EpidemicNetworkState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Susceptible,
    Infected,
    Resistant,
    // Immune,
}

#[derive(Clone, Copy)]
pub struct NetNode {
    pub id: u32,
    pub loc: Real2D,
    pub status: NodeStatus,
    pub virus_detected: bool,
}

impl NetNode {
    pub fn new(id: u32, loc: Real2D, init_status: NodeStatus) -> Self {
        NetNode {
            id,
            loc,
            status: init_status,
            virus_detected: false,
        }
    }
}

impl Agent for NetNode {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any()
            .downcast_ref::<EpidemicNetworkState>()
            .unwrap();

        let mut rng = rand::thread_rng();
        match self.status {
            NodeStatus::Infected => {
                if rng.gen_bool(state.recovery as f64) {
                    self.status = NodeStatus::Resistant;
                }
            }
            NodeStatus::Susceptible => {
                let neighborhood = state.network.get_edges(*self);
                if neighborhood.is_none() {
                    return;
                };

                let neighborhood = neighborhood.unwrap();
                // for each neighbor check if it is infected, if so check virus_spread
                // and become infected
                for edge in &neighborhood {
                    let node = state.network.get_object(edge.v).unwrap();
                    match node.status {
                        NodeStatus::Infected => {
                            if rng.gen_bool(state.spread as f64) {
                                self.status = NodeStatus::Infected;
                                // increase count of how many nodes node has infected
                                state.infected_nodes.lock().unwrap()[edge.v as usize] += 1;
                                break;
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            NodeStatus::Resistant => {}
        }
        state.network.update_node(*self);
        state.field1.set_object_location(*self, self.loc);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Hash for NetNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for NetNode {}

impl PartialEq for NetNode {
    fn eq(&self, other: &NetNode) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for NetNode {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for NetNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {} status {}", self.id, self.loc, self.status)
    }
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeStatus::Susceptible => write!(f, "Susceptible"),
            NodeStatus::Infected => write!(f, "Infected"),
            NodeStatus::Resistant => write!(f, "Resistant"),
        }
    }
}
