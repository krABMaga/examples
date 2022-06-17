use crate::model::sheep::Sheep;
use crate::model::state::WsgState;
use crate::model::wolf::Wolf;
use crate::visualization::sheep_vis::SheepVis;
use crate::visualization::wolf_vis::WolfVis;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct VisState;

impl VisualizationState<WsgState> for VisState {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        state: &mut WsgState,
        _schedule: &mut Schedule,
        sim: &mut SimulationDescriptor,
    ) {
        Self::generate_grass(&state, sprite_render_factory, commands, sim);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &WsgState,
    ) -> Option<Box<dyn AgentRender>> {
        if let Some(wolf) = agent.downcast_ref::<Wolf>() {
            Some(Box::new(WolfVis { id: wolf.id }))
        } else {
            let sheep = agent.downcast_ref::<Sheep>().unwrap();
            Some(Box::new(SheepVis { id: sheep.id }))
        }
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<WsgState>().unwrap();
        if let Some(_wolf_vis) = agent_render.downcast_ref::<WolfVis>() {
            match state.wolves_grid.get(&Wolf::new(
                agent_render.get_id(),
                Int2D { x: 0, y: 0 },
                0.,
                0.,
                0.,
            )) {
                Some(matching_agent) => Some(Box::new(matching_agent)),
                None => None,
            }
        } else {
            match state.sheep_grid.get(&Sheep::new(
                agent_render.get_id(),
                Int2D { x: 0, y: 0 },
                0.,
                0.,
                0.,
            )) {
                Some(matching_agent) => Some(Box::new(matching_agent)),
                None => None,
            }
        }
    }

    fn before_render(
        &mut self,
        state: &mut WsgState,
        _schedule: &Schedule,
        commands: &mut Commands,
        asset_factory: &mut AssetHandleFactoryResource,
    ) {
        // let new_sheep = state.new_sheep.lock().unwrap();
        // for sheep in &*new_sheep {
        for sheep in &state.new_sheep {
            //let boxed_agent = &(*sheep).as_agent();
            let boxed_agent = &sheep.as_agent();
            let boxed_state = Box::new(state.as_state());
            let sheep_vis = self.get_agent_render(boxed_agent, state);
            let SpriteType::Emoji(emoji_code) =
                sheep_vis.unwrap().sprite(boxed_agent, &boxed_state);
            let sprite_render = asset_factory.get_emoji_loader(emoji_code);
            self.setup_agent_graphics(
                boxed_agent,
                self.get_agent_render(boxed_agent, state).unwrap(),
                sprite_render,
                commands,
                &boxed_state,
            );
        }

        // let new_wolves = state.new_wolves.lock().unwrap();
        // for wolf in &*new_wolves {
        for wolf in &state.new_wolves {
            let boxed_wolf = &wolf.as_agent();
            let boxed_state = Box::new(state.as_state());
            let wolf_vis = self.get_agent_render(boxed_wolf, state);
            let SpriteType::Emoji(emoji_code) = wolf_vis.unwrap().sprite(boxed_wolf, &boxed_state);
            let sprite_render = asset_factory.get_emoji_loader(emoji_code);
            self.setup_agent_graphics(
                boxed_wolf,
                self.get_agent_render(boxed_wolf, state).unwrap(),
                sprite_render,
                commands,
                &boxed_state,
            );
        }
    }
}

impl VisState {
    fn generate_grass(
        state: &WsgState,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state
            .grass_field
            .render(&mut *sprite_render_factory, commands, sim);
    }
}
