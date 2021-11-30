use crate::model::bird::Bird;
use crate::model::state::Flocker;
use rust_ab::bevy::prelude::{Quat, Transform, Visible};
use rust_ab::engine::agent::Agent;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::{AgentRender, SpriteType};
use std::f32::consts::PI;

pub struct BirdVis {
    pub(crate) id: u32,
}

impl AgentRender for BirdVis {
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("bird"))
    }

    fn position(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state.as_any().downcast_ref::<Flocker>().unwrap();
        let agent = agent.downcast_ref::<Bird>().unwrap();
        let loc = state.field1.get_location(*agent);
        match loc {
            Some(loc) => (loc.x as f32, loc.y as f32, 0.),
            None => (agent.pos.x as f32, agent.pos.y as f32, 0.),
        }
    }

    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.1, 0.1)
    }

    /// The bird emoji points to left by default, so we calculate the rotation
    /// and offset it by pi radians
    fn rotation(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        let concrete_agent = agent.downcast_ref::<Bird>().unwrap();
        let rotation = if concrete_agent.last_d.x == 0. || concrete_agent.last_d.y == 0. {
            0.
        } else {
            concrete_agent.last_d.y.atan2(concrete_agent.last_d.x)
        };
        (rotation + PI) as f32
    }

    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visible,
    ) {
        let (pos_x, pos_y, z) = self.position(agent, state);
        let rotation = self.rotation(agent, state);
        let (scale_x, scale_y) = self.scale(agent, state);

        let translation = &mut transform.translation;
        translation.x = pos_x;
        translation.y = pos_y;
        translation.z = z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
