use rust_ab::bevy::prelude::Commands;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::AgentRender;
use rust_ab::visualization::asset_handle_factory::AssetHandleFactoryResource;
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::visualization_state::VisualizationState;

use crate::model::bird::Bird;
use crate::model::state::Flocker;
use crate::visualization::bird_vis::BirdVis;

#[derive(Clone)]
pub struct VisState;

impl VisualizationState<Flocker> for VisState {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _sprite_render_factory: &mut AssetHandleFactoryResource,
        _state: &mut Flocker,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &Flocker,
    ) -> Option<Box<dyn AgentRender>> {
        // In a multi-agent model, you may want to do so
        // if let Some(bird) = agent.as_any().downcast_ref::<Bird>() {Box::new(BirdVis);} etc...
        // We only have one agent here so we can directly return the correct AgentRender
        Some(Box::new(BirdVis {
            id: agent.downcast_ref::<Bird>().unwrap().id,
        }))
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        // TODO we don't just need the agent associated to the agent render, we need the correct one so that we
        // can access data such as location etc...
        let state = state.as_any().downcast_ref::<Flocker>().unwrap();
        match state.field1.get(&Bird::new(
            agent_render.get_id(),
            Real2D { x: 0., y: 0. },
            Real2D { x: 0., y: 0. },
        )) {
            Some(matching_agent) => Some(Box::new(*matching_agent)),
            None => None,
        }
    }
}
