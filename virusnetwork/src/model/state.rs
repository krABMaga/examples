use crate::model::node::NetNode;

use rust_ab::engine::field::{field::Field, field_2d::Field2D, network::Network};
use rust_ab::engine::state::State;

pub struct EpidemicNetworkState {
    pub step: u128,
    pub field1: Field2D<NetNode>,
    pub network: Network<NetNode, String>,
}

impl EpidemicNetworkState {
    pub fn new(w: f64, h: f64, d: f64, t: bool) -> EpidemicNetworkState {
        EpidemicNetworkState {
            step: 0,
            field1: Field2D::new(w, h, d, t),
            network: Network::new(false),
        }
    }
}

impl State for EpidemicNetworkState{
    fn update(&mut self){
        self.field1.update();
    }
}
