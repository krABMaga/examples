use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State as StateTrait;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

use crate::model::ant::Ant;
use crate::model::state::*;
use crate::visualization::ant::AntVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;

#[derive(Clone, Resource)]
pub struct VisState;

impl VisualizationState<ModelState> for VisState {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_factory: &mut AssetHandleFactoryResource,
        state: &mut ModelState,
        _schedule: &mut Schedule,
        sim: &mut SimulationDescriptor,
    ) {
        state.to_home_grid.render(sprite_factory, commands, sim);
        state.to_food_grid.render(sprite_factory, commands, sim);
        SparseGrid2D::<Item>::init_graphics_grid(sprite_factory, commands, state);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &ModelState,
    ) -> Option<Box<dyn AgentRender>> {
        if let Some(ant) = agent.downcast_ref::<Ant>() {
            Some(Box::new(AntVis { id: ant.id }))
        } else {
            None
        }
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn StateTrait>,
    ) -> Option<Box<dyn Agent>> {
        let state = state.as_any().downcast_ref::<ModelState>().unwrap();
        if let Some(_ant_vis) = agent_render.downcast_ref::<AntVis>() {
            match state.ants_grid.get(&Ant::new(
                agent_render.get_id(),
                Int2D { x: 0, y: 0 },
                false,
                0.,
            )) {
                Some(matching_agent) => Some(Box::new(matching_agent)),
                None => None,
            }
        } else {
            None
        }
    }

    fn before_render(
        &mut self,
        _state: &mut ModelState,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }
}

impl VisState {}
