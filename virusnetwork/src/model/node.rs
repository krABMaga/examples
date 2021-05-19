use rust_ab::rand;
use rust_ab::{
    engine::{
        agent::Agent,
        field::field::Field,
        location::{Location2D, Real2D},
    },
    rand::Rng,
};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::model::state::EpidemicNetworkState;
use crate::model::state::{GAIN_RESISTENCE_CHANCE, RECOVERY_CHANCE, VIRUS_CHECK_FREQUENCY, VIRUS_SPREAD_CHANCE};

#[derive(Clone, Copy, Debug)]
pub enum NodeStatus {
    Susceptible,
    Infected,
    Resistent,
}

#[derive(Clone, Copy)]
pub struct NetNode {
    pub id: u128,
    pub pos: Real2D,
    pub status: NodeStatus,
    pub virus_detected: bool,
}

impl NetNode {
    pub fn new(id: u128, pos: Real2D, init_status: NodeStatus) -> Self {
        NetNode {
            id,
            pos,
            status: init_status,
            virus_detected: false,
        }
    }


    fn spread_virus(&mut self, state: &EpidemicNetworkState) {
        let neighborhood = state.network.getEdges(&self);
        if neighborhood.is_none() {
            return;
        };
    
        let mut rng = rand::thread_rng();
        println!("Node{} spreading virus", self.id);
        for edge in neighborhood.unwrap() {
            if rng.gen_bool(VIRUS_SPREAD_CHANCE) {
                let mut victim = edge.v;
    
                match victim.status {
                    NodeStatus::Susceptible => {
                        println!("{} Infected by {}", self.id, victim.id);
                        victim.status = NodeStatus::Infected;
    
                        //update
                        state.network.addNode(&victim);
                        state.field1.set_object_location(victim, victim.pos);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
    
    fn recovery_attempt(&mut self, state: &EpidemicNetworkState) -> NodeStatus{
        let mut rng = rand::thread_rng();
        if rng.gen_bool(RECOVERY_CHANCE) {
            self.virus_detected = false;

            if rng.gen_bool(GAIN_RESISTENCE_CHANCE){    
                return NodeStatus::Resistent
            }
            else {
                return NodeStatus::Susceptible
            }
        }
    
        NodeStatus::Infected
    }

    fn routine(&mut self, state: &EpidemicNetworkState){
        if !self.virus_detected {
            //Scan Virus
            let mut rng = rand::thread_rng();
            self.virus_detected = rng.gen_bool(VIRUS_CHECK_FREQUENCY);
        }

        if self.virus_detected {
            self.status = self.recovery_attempt(state);
        } 
    }

}



impl Agent for NetNode {
    type SimState = EpidemicNetworkState;

    fn step(&mut self, state: &EpidemicNetworkState) {
        //println!("STEP {}: Node{}, status:{:?}", state.step, self.id, self.status);
        match self.status {
            NodeStatus::Infected => {
                self.spread_virus( state);
                self.routine(state);
            }
            NodeStatus::Susceptible => {
               // self.routine(state);
            }
            _ => {}
        }
    }
}

impl Hash for NetNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
        //    state.write_u128(self.id);
        //    state.finish();
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
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for NetNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
