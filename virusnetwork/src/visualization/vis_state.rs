use crate::model::{node::*, state::EpidemicNetworkState};
use crate::visualization::node::NetNodeVis;
use krABMaga::bevy::prelude::Commands;
use krABMaga::engine::agent::Agent;
use krABMaga::engine::schedule::*;
use krABMaga::engine::state::State;
use krABMaga::visualization::agent_render::AgentRender;
use krABMaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krABMaga::visualization::fields::network::NetworkRender;
use krABMaga::visualization::simulation_descriptor::SimulationDescriptor;
use krABMaga::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct VisState;

impl VisualizationState<EpidemicNetworkState> for VisState {
    fn on_init(
        &self,
        commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        state: &mut EpidemicNetworkState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        EpidemicNetworkState::init_network_graphics(state, commands)
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
