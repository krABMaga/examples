use crate::model::map::Map;
use crate::model::person::Person;
use crate::visualization::person_vis::PersonVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};
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

    fn before_render(
        &mut self,
        state: &mut Map,
        _schedule: &Schedule,
        commands: &mut Commands,
        asset_factory: &mut AssetHandleFactoryResource,
    ) {
        // let new_sheep = state.new_sheep.lock().unwrap();
        // for sheep in &*new_sheep {
        for person in &state.people {
            //let boxed_agent = &(*sheep).as_agent();
            let boxed_agent = &person.as_agent();
            let boxed_state = Box::new(state.as_state());
            let person_vis = self.get_agent_render(boxed_agent, state);
            let SpriteType::Emoji(emoji_code) =
                person_vis.unwrap().sprite(boxed_agent, &boxed_state);
            let sprite_render = asset_factory.get_emoji_loader(emoji_code);
            self.setup_agent_graphics(
                boxed_agent,
                self.get_agent_render(boxed_agent, state).unwrap(),
                sprite_render,
                commands,
                &boxed_state,
            );
        }
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<Map>().unwrap();
        match state.field.get(&Person {
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
