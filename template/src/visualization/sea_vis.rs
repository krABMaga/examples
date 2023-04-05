use crate::model::crab::Crab;
use crate::model::sea::Sea;
use crate::visualization::crab_vis::CrabVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
pub struct SeaVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<Sea> for SeaVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut Sea,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Sea,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(CrabVis {
            id: agent.downcast_ref::<Crab>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<Sea>().unwrap();
        match state.field.get(&Crab {
            id: agent_render.get_id(),
            loc: Real2D { x: 0., y: 0. },
            last_d: Real2D { x: 0., y: 0. },
            dir_x: 0.,
            dir_y: 0.,
        }) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        }
    }
}
