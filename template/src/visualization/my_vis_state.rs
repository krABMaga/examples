use rust_ab::bevy::ecs::prelude::{Commands, ResMut};
use rust_ab::engine::schedule::Schedule;
use rust_ab::visualization::on_state_init::OnStateInit;
use rust_ab::visualization::renderable::{Render, SpriteType};
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;

use crate::model::my_agent::MyAgent;
use crate::model::my_state::MyState;

pub struct MyVisState;

/// Define how the simulation should be bootstrapped. Agents should be created here.
impl OnStateInit<MyAgent> for MyVisState {
    fn on_init(
        &self,
        mut commands: Commands,
        mut sprite_render_factory: SpriteFactoryResource,
        state: ResMut<MyState>,
        mut schedule: ResMut<Schedule<MyAgent>>,
        _sim: ResMut<SimulationDescriptor>) {
        let my_agent = MyAgent{id: 1};
        // Put the agent in your state
        schedule.schedule_repeating(my_agent, 0., 0);

        let SpriteType::Emoji(emoji_code) = my_agent.sprite();
        let sprite_render =
            sprite_render_factory.get_emoji_loader(emoji_code);
        my_agent.setup_graphics(sprite_render, &mut commands, &state);
    }
}