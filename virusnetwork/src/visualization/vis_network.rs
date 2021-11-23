use crate::model::node::NetNode;
use crate::model::state::EpidemicNetworkState;
use rust_ab::bevy::prelude::*;
use rust_ab::engine::fields::network::{Edge, Network};
use rust_ab::visualization::fields::network::DrawMode;
use rust_ab::visualization::fields::network::{EdgeRenderInfo, LineType, NetworkRender};

impl NetworkRender<NetNode, String, EpidemicNetworkState> for EpidemicNetworkState {
    fn get_network(state: &EpidemicNetworkState) -> &Network<NetNode, String> {
        &state.network
    }

    fn get_edge_info(edge: &Edge<String>, network: &Network<NetNode, String>) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            draw_mode: DrawMode::stroke_1px(),
            source_loc: network.get_object(edge.u).unwrap().loc,
            target_loc: network.get_object(edge.v).unwrap().loc,
            line_type: LineType::Line,
        }
    }
}
