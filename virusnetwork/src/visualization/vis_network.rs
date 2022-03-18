use crate::model::node::NetNode;
use crate::model::state::EpidemicNetworkState;
use rust_ab::bevy::prelude::*;
use rust_ab::engine::fields::network::{Edge, Network};
use rust_ab::engine::location::Real2D;
use rust_ab::visualization::fields::network::{EdgeRenderInfo, NetworkRender};

impl NetworkRender<NetNode, String, EpidemicNetworkState> for EpidemicNetworkState {
    fn get_network(state: &EpidemicNetworkState) -> &Network<NetNode, String> {
        &state.network
    }

    fn get_edge_info(edge: &Edge<String>, network: &Network<NetNode, String>) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            line_width: 1.,
            source_loc: network.get_object(edge.u).unwrap().loc,
            target_loc: network.get_object(edge.v).unwrap().loc,
            is_static: true
        }
    }

    fn get_loc(network: &Network<NetNode, String>, node: u32) -> Real2D {
        network.get_object(node).unwrap().loc
    }
}
