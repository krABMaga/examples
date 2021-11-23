use crate::model::state::WsgState;
use crate::model::wolf::Wolf;
use rust_ab::bevy::prelude::{Quat, Transform, Visible};
use rust_ab::engine::agent::Agent;
use rust_ab::engine::location::Int2D;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::{AgentRender, SpriteType};

pub struct WolfVis {
    pub id: u32,
}

impl AgentRender for WolfVis {
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("wolf"))
    }

    fn position(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state.as_any().downcast_ref::<WsgState>().unwrap();
        let agent = agent.downcast_ref::<Wolf>().unwrap();
        let loc = state.wolves_grid.get_location(*agent);
        match loc {
            Some(pos) => (pos.x as f32, pos.y as f32, 1.),
            None => (agent.loc.x as f32, agent.loc.y as f32, 1.),
        }
    }

    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.002, 0.002)
    }

    fn rotation(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        let agent = agent.downcast_ref::<Wolf>().unwrap();
        let rotation = if let Some(Int2D { x, y }) = agent.last {
            ((y - agent.loc.y) as f32).atan2((x - agent.loc.x) as f32)
        } else {
            0.
        };
        rotation
    }

    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visible,
    ) {
        let (pos_x, pos_y, z) = self.position(agent, state);
        let (scale_x, scale_y) = self.scale(agent, state);
        let rotation = self.rotation(agent, state);
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
