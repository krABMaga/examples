use crate::model::forest::Forest;
use crate::Tree;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct ForestVis;

impl VisualizationState<Forest> for ForestVis {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_factory: &mut AssetHandleFactoryResource,
        state: &mut Forest,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        state.field.update();
        DenseGrid2D::<Tree>::init_graphics_grid(sprite_factory, commands, state);
    }

    fn before_render(
        &mut self,
        _state: &mut Forest,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }

    fn get_agent_render(
        &self,
        _agent: &Box<dyn Agent>,
        _state: &Forest,
    ) -> Option<Box<dyn AgentRender>> {
        None
    }

    fn get_agent(
        &self,
        _agent_render: &Box<dyn AgentRender>,
        _state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        None
    }
}

impl ForestVis {}
