use crate::model::{node::*, state::EpidemicNetworkState};
use crate::visualization::node::NetNodeVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::schedule::*;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::network::NetworkRender;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
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
