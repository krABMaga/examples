use rust_ab::bevy::prelude::Color;
use rust_ab::bevy_canvas::DrawMode;
use rust_ab::engine::field::network::{Edge, Network};
use rust_ab::visualization::field::network::{EdgeRenderInfo, LineType, NetworkRender};

use crate::model::epidemic_network::EpidemicNetwork;
use crate::model::node::NetNode;
use crate::model::state::EpidemicNetworkState;

impl NetworkRender<NetNode, String, NetNode> for EpidemicNetwork {
    fn get_network(state: &EpidemicNetworkState) -> &Network<NetNode, String> {
        &state.network.network
    }

    fn get_edge_info(edge: &Edge<NetNode, String>) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            draw_mode: DrawMode::stroke_1px(),
            source_loc: edge.u.pos,
            target_loc: edge.v.pos,
            line_type: LineType::Line,
        }
    }
}