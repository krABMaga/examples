use crate::model::eater::Eater;
use crate::model::state::Environment;
use crate::visualization::eater_vis::EaterVis;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Resource;
use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;
use krabmaga::Rng;

#[derive(Clone, Resource)]
pub struct EnvironmentVis;

/// Define how the simulation should be bootstrapped. Agents should be created here.

impl VisualizationState<Environment> for EnvironmentVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut Environment,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        Self::generate_patches(_state, _sprite_render_factory, _commands, _sim);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Environment,
    ) -> Option<Box<dyn AgentRender>> {
        Some(Box::new(EaterVis {
            id: agent.downcast_ref::<Eater>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let mut rng = krabmaga::rand::thread_rng();
        let state = state.as_any().downcast_ref::<Environment>().unwrap();
        match state.eaters.get(&Eater {
            id: agent_render.get_id(),
            position: Int2D {
                x: rng.gen_range(0..20),
                y: rng.gen_range(0..20),
            },
            vision: 4,
            metabolism: 50,
            age: 0,
            max_age: 20,
            wealth: 5,
        }) {
            Some(matching_agent) => Some(Box::new(matching_agent)),
            None => None,
        }
    }
}
impl EnvironmentVis {
    fn generate_patches(
        state: &Environment,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state
            .field
            .render(&mut *sprite_render_factory, commands, sim);
    }
}
