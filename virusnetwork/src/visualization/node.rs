use rust_ab::bevy::prelude::{Transform, Visible};

use crate::model::node::*;
use crate::model::state::EpidemicNetworkState;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::{AgentRender, SpriteType};

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
            NodeStatus::Resistent => SpriteType::Emoji(String::from("large_blue_circle")),
        }
    }

    /// The position must always be fetched through the state, since that will be the one actually updated
    /// by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    /// put on a lower z-axis.
    fn position(&self, agent: &Box<dyn Agent>, state: &Box<&dyn State>) -> (f32, f32, f32) {
        let state = state
            .as_any()
            .downcast_ref::<EpidemicNetworkState>()
            .unwrap();
        let agent = agent.downcast_ref::<NetNode>().unwrap();
        let loc = state.field1.get_location(*agent);
        match loc {
            Some(loc) => (loc.x as f32, loc.y as f32, 0.),
            None => (agent.loc.x as f32, agent.loc.y as f32, 0.),
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

    /// Simply update the transform based on the position chosen
    fn update(
        &mut self,
        _agent: &Box<dyn Agent>,
        _transform: &mut Transform,
        _state: &Box<&dyn State>,
        _visible: &mut Visible,
    ) {
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
