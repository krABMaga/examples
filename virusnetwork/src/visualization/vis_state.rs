use crate::model::{node::*, state::EpidemicNetworkState};
use crate::visualization::node::NetNodeVis;
use rust_ab::bevy::prelude::Commands;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::schedule::*;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::AgentRender;
use rust_ab::visualization::asset_handle_factory::AssetHandleFactoryResource;
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct VisState;

impl VisualizationState<EpidemicNetworkState> for VisState {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut EpidemicNetworkState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &EpidemicNetworkState,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(NetNodeVis {
            id: agent.downcast_ref::<NetNode>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state
            .as_any()
            .downcast_ref::<EpidemicNetworkState>()
            .unwrap();
        match state.network.get_object(agent_render.get_id()) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None,
        }
    }
}
