use krabmaga::bevy::prelude::{Transform, Vec3, Visibility};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

use crate::model::body::BodyData;
use crate::model::state::NBodyState;

/// Side (in pixels) of assets/emojis/sun.png and planet.png.
const SPRITE_PX: f32 = 64.0;
const OFFSCREEN: f32 = -100_000.0;

fn data(state: &Box<&dyn State>, id: u32) -> Option<BodyData> {
    let s = state.as_any().downcast_ref::<NBodyState>()?;
    s.bodies.get(id as usize).copied()
}

#[derive(Clone)]
pub struct BodyVis {
    pub id: u32,
}

impl AgentRender for BodyVis {
    fn sprite(&self, _agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> SpriteType {
        let star = data(state, self.id).map_or(false, |d| d.is_star);
        SpriteType::Emoji(String::from(if star { "sun" } else { "planet" }))
    }

    fn location(&self, _agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        data(state, self.id)
            .map(|d| (d.pos.x, d.pos.y, if d.is_star { 1.0 } else { 0.0 }))
            .unwrap_or((OFFSCREEN, OFFSCREEN, 0.0))
    }

    fn scale(&self, _agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32) {
        let k = data(state, self.id).map_or(0.0, |d| 2.0 * d.radius / SPRITE_PX);
        (k, k)
    }

    fn rotation(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        0.0
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn update(
        &mut self,
        _agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        visible: &mut Visibility,
    ) {
        if let Some(d) = data(state, self.id) {
            let k = 2.0 * d.radius / SPRITE_PX;
            transform.translation.x = d.pos.x;
            transform.translation.y = d.pos.y;
            transform.scale = Vec3::new(k, k, 1.0);
            *visible = Visibility::Visible;
        }
    }
}
