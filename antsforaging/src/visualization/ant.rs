use crate::model::ant::Ant;
use crate::model::state::ModelState;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::{Component, Quat, Transform, Visibility};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

pub struct AntVis {
    pub id: u32,
}
impl Component for AntVis {
    type Storage = TableStorage;
}
impl AgentRender for AntVis {
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("ant"))
    }

    // The location must always be fetched through the state, since that will be the one actually updated
    // by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    // put on a lower z-axis.
    fn location(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state.as_any().downcast_ref::<ModelState>().unwrap();
        let agent = agent.downcast_ref::<Ant>().unwrap();
        let loc = state.ants_grid.get_location(*agent);
        match loc {
            Some(loc) => (loc.x as f32, loc.y as f32, 1.),
            None => (agent.loc.x as f32, agent.loc.y as f32, 1.),
        }
    }

    // Emojis are 64x64, way too big for our simulation
    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.1, 0.1)
    }

    fn rotation(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        let agent = agent.downcast_ref::<Ant>().unwrap();
        if let Some(Int2D { x, y }) = agent.last {
            ((y - agent.loc.y) as f32).atan2((x - agent.loc.x) as f32)
        } else {
            0.
        }
    }

    /// Simply update the transform based on the location chosen
    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        state: &Box<&dyn State>,
        _visible: &mut Visibility,
    ) {
        let (loc_x, loc_y, z) = self.location(agent, state);
        let (scale_x, scale_y) = self.scale(agent, state);

        let rotation = self.rotation(agent, state);

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
