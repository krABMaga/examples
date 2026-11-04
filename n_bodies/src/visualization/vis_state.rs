use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::prelude::{Commands, Quat, Resource, Sprite, Transform, Vec3};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, AgentRenderComponent};
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;
use krabmaga::visualization::wrappers::SimulationRenderEntity;

use crate::model::body::Body;
use crate::model::state::NBodyState;
use crate::visualization::body_vis::BodyVis;
use crate::visualization::colors::body_color;

#[derive(Clone, Resource)]
pub struct NBodyVis;

impl VisualizationState<NBodyState> for NBodyVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _factory: &mut AssetHandleFactoryResource,
        _state: &mut NBodyState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &NBodyState,
    ) -> Option<Box<dyn AgentRender>> {
        let b = agent.downcast_ref::<Body>()?;
        Some(Box::new(BodyVis { id: b.id }))
    }

    fn get_agent(
        &self,
        agent_render: &dyn AgentRender,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let s = state.as_any().downcast_ref::<NBodyState>()?;
        let id = agent_render.get_id();
        if (id as usize) < s.bodies.len() {
            Some(Box::new(Body { id }))
        } else {
            None
        }
    }

    // Same as the default implementation of the trait, but every planet
    // sprite is tinted with the colour of its trail.
    fn setup_agent_graphics(
        &self,
        agent: &Box<dyn Agent>,
        agent_render: Box<dyn AgentRender>,
        mut sprite: Sprite,
        commands: &mut Commands,
        state: &Box<&dyn State>,
    ) {
        let id = agent_render.get_id();
        let is_star = state
            .as_any()
            .downcast_ref::<NBodyState>()
            .and_then(|s| s.bodies.get(id as usize).map(|b| b.is_star))
            .unwrap_or(false);
        if !is_star {
            sprite.color = body_color(id, 1.0);
        }

        let (x, y, z) = agent_render.location(agent, state);
        let (sx, sy) = agent_render.scale(agent, state);
        let rotation = agent_render.rotation(agent, state);

        let mut transform = Transform::from_translation(Vec3::new(x, y + 0.5, z));
        transform.scale.x = sx;
        transform.scale.y = sy;
        transform.rotation = Quat::from_rotation_z(rotation);

        commands
            .spawn((sprite, transform))
            .insert(AgentRenderComponent(agent_render))
            .insert(SimulationRenderEntity);
    }
}
