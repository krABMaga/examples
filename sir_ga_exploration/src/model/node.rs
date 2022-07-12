use krabmaga::engine::state::State;
use krabmaga::{engine::agent::Agent, rand::Rng};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::model::state::EpidemicNetworkState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Susceptible,
    Infected,
    Resistant,
}

#[derive(Clone, Copy)]
pub struct NetNode {
    pub id: u32,
    pub status: NodeStatus,
    pub virus_detected: bool,
}

impl NetNode {
    pub fn new(id: u32, init_status: NodeStatus) -> Self {
        NetNode {
            id,
            status: init_status,
            virus_detected: false,
        }
    }
}

impl Agent for NetNode {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any_mut()
            .downcast_mut::<EpidemicNetworkState>()
            .unwrap();
        match self.status {
            NodeStatus::Infected => {
                if state.rng.lock().unwrap().gen_bool(state.recovery as f64) {
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
                            let spread;
                            if state.step > state.day {
                                spread = state.spread2;
                            } else {
                                spread = state.spread;
                            }
                            if state.rng.lock().unwrap().gen_bool(spread as f64) {
                                self.status = NodeStatus::Infected;
                                // increase count of how many nodes node has infected
                                state.infected_nodes[node.id as usize] += 1;
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

impl fmt::Display for NetNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} status {}", self.id, self.status)
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
