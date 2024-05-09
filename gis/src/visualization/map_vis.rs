use crate::model::map::Map;
use crate::model::person::Person;
use crate::visualization::person_vis::PersonVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone, Resource)]
pub struct MapVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<Map> for MapVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut Map,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Map,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(PersonVis {
            id: agent.downcast_ref::<Person>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<Map>().unwrap();
        /*  match state.field(&Person {
            id: agent_render.get_id(),
            loc: Int2D { x: 0, y: 0 },
            last_d: Int2D { x: 0, y: 0 },
            dir_x: 0.,
            dir_y: 0.,
        }) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        } */
        None
    }
}
