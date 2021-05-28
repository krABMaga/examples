use rust_ab::engine::field::network::Network;

use crate::model::node::NetNode;

pub struct EpidemicNetwork {
    pub network: Network<NetNode, String>,
}

impl EpidemicNetwork {
    pub fn new() -> EpidemicNetwork {
        EpidemicNetwork {
            network: Network::new(false),
        }
    }
}