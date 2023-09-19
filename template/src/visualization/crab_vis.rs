use crate::model::{crab::Crab, sea::Sea};
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::prelude::{Component, Quat, Transform, Visibility};
use krabmaga::{
    engine::{agent::Agent, state::State},
    visualization::agent_render::{AgentRender, SpriteType},
};
use std::f32::consts::PI;

#[derive(Component)]
pub struct CrabVis {
    pub(crate) id: u32,
}

/// Define how your agent should be rendered here.
impl AgentRender for CrabVis {
    /// Specify the assets to use. Swap "bird" with the file name of whatever emoji you want to use.
    /// Be sure to also copy the asset itself in the assets/emojis folder. In future, this limitation will
    /// be removed.
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("crab"))
    }

    /// Specify where the agent should be rendered in the window.
    fn location(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state.as_any().downcast_ref::<Sea>().unwrap();
        let agent = agent.downcast_ref::<Crab>().unwrap();
        let loc = state.field.get_location(*agent);
        match loc {
            Some(loc) => (loc.x, loc.y, 0.),
            None => (agent.loc.x, agent.loc.y, 0.),
        }
    }

    /// Specify how much the texture should be scaled by. A common scale is (0.1, 0.1).
    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.2, 0.2)
    }

    /// Define the degrees in radians to rotate the texture by.
    fn rotation(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        let concrete_agent = agent.downcast_ref::<Crab>().unwrap();
        let rotation = if concrete_agent.last_d.x == 0. || concrete_agent.last_d.y == 0. {
            0.
        } else {
            concrete_agent.last_d.y.atan2(concrete_agent.last_d.x)
        };
        rotation - PI
    }

    /// Specify the code to execute for each frame, for each agent.
    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visibility,
    ) {
        // This snippet updates the agent location, scale and rotation for each frame.
        let (loc_x, loc_y, z) = self.location(agent, state);
        let rotation = self.rotation(agent, state);
        let (scale_x, scale_y) = self.scale(agent, state);

        let translation = &mut transform.translation;
        translation.x = loc_x;
        translation.y = loc_y;
        translation.z = z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
