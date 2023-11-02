use krabmaga::bevy::prelude::{Component, Transform, Visibility};

use crate::model::node::*;
use crate::model::state::EpidemicNetworkState;
use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};

#[derive(Component)]
pub struct NetNodeVis {
    pub id: u32,
}

impl AgentRender for NetNodeVis {
    fn sprite(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> SpriteType {
        let agent = agent.downcast_ref::<NetNode>().unwrap();
        let state = state
            .as_any()
            .downcast_ref::<EpidemicNetworkState>()
            .unwrap();

        let status = if let Some(updated_agent) = state.network.get_object(agent.id) {
            updated_agent.status
        } else {
            agent.status
        };
        match status {
            NodeStatus::Susceptible => SpriteType::Emoji(String::from("white_circle")),
            NodeStatus::Infected => SpriteType::Emoji(String::from("red_circle")),
            NodeStatus::Resistant => SpriteType::Emoji(String::from("large_blue_circle")),
        }
    }

    /// The location must always be fetched through the state, since that will be the one actually updated
    /// by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    /// put on a lower z-axis.
    fn location(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state
            .as_any()
            .downcast_ref::<EpidemicNetworkState>()
            .unwrap();
        let agent = agent.downcast_ref::<NetNode>().unwrap();
        let loc = state.field1.get_location(*agent);
        match loc {
            Some(loc) => (loc.x, loc.y, 0.),
            None => (agent.loc.x, agent.loc.y, 0.),
        }
    }

    /// Emojis are 64x64, way too big for our simulation
    fn scale(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32) {
        (0.3, 0.3)
    }

    /// No rotation is needed
    fn rotation(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        0.
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
        let translation = &mut transform.translation;
        translation.x = loc_x;
        translation.y = loc_y;
        translation.z = z;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
