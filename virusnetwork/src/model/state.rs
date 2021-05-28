use rust_ab::engine::field::{field::Field, field_2d::Field2D};
use rust_ab::engine::state::State;

use crate::model::epidemic_network::EpidemicNetwork;
use crate::model::node::NetNode;

///Initial infected nodes
pub static INITIAL_INFECTED_PROB: f64 = 0.01;
//#[param=integer,min=2,max=6,n=1000,distribution=uniform]
pub static INIT_EDGE: u128 = 2;
//#[param=float,min=0.1,max=0.8,n=1000,distribution=normal]
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
//#[param=float,min=0.2,max=0.8,n=1000,distribution=normal]
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
//#[param=float,min=0.2,max=0.8,n=1000,distribution=normal]
pub static RECOVERY_CHANCE: f64 = 0.30;
//#[param=float,min=0.2,max=0.8,n=1000,distribution=normal]
pub static GAIN_RESISTENCE_CHANCE: f64 = 0.20;


pub struct EpidemicNetworkState {
    pub step: usize,
    pub field1: Field2D<NetNode>,
    pub network: EpidemicNetwork,
}

impl EpidemicNetworkState {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(w, h, d, t),
            network: EpidemicNetwork::new(),
        }
    }
}

impl State for EpidemicNetworkState {
    fn update(&mut self, step: usize) {
        self.field1.update();
        self.step = step;
    }
}
