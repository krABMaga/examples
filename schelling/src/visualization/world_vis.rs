use crate::model::world::World;
use crate::Patch;
use krABMaga::bevy::prelude::Commands;
use krABMaga::engine::agent::Agent;
use krABMaga::engine::fields::field::Field;
use krABMaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krABMaga::engine::schedule::Schedule;
use krABMaga::engine::state::State;
use krABMaga::visualization::agent_render::AgentRender;
use krABMaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krABMaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krABMaga::visualization::simulation_descriptor::SimulationDescriptor;
use krABMaga::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct WorldVis;

impl VisualizationState<World> for WorldVis {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_factory: &mut AssetHandleFactoryResource,
        state: &mut World,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        state.field.update();
        SparseGrid2D::<Patch>::init_graphics_grid(sprite_factory, commands, state);
    }

    fn before_render(
        &mut self,
        _state: &mut World,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }

    fn get_agent_render(
        &self,
        _agent: &Box<dyn Agent>,
        _state: &World,
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

impl WorldVis {}
