use crate::model::bird::Bird;
use crate::model::boids_state::{BoidsState, HEIGHT, WIDTH};
use crate::NUM_AGENT;
use rust_ab::bevy::prelude::{Commands, ResMut};
use rust_ab::engine::location::Real2D;
use rust_ab::visualization::on_state_init::OnStateInit;
use rust_ab::visualization::renderable::{Render, SpriteType};
use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;
use rust_ab::rand;
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::rand::Rng;
use rust_ab::engine::schedule::Schedule;

pub struct VisState;

impl OnStateInit<Bird> for VisState {
    fn on_init(
        &self,
        mut commands: Commands,
        mut sprite_render_factory: SpriteFactoryResource,
        state: ResMut<BoidsState>,
        mut schedule: ResMut<Schedule<Bird>>,
        mut _sim: ResMut<SimulationDescriptor>
    ) {
        let mut rng = rand::thread_rng();

        for bird_id in 0..NUM_AGENT {
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let pos = Real2D {
                x: WIDTH * r1,
                y: HEIGHT * r2,
            };
            let bird = Bird::new(bird_id, pos, last_d);
            state.field1.set_object_location(bird, pos);
            schedule.schedule_repeating(bird, 0., 0);

            let SpriteType::Emoji(emoji_code) = bird.sprite();
            let sprite_render =
                sprite_render_factory.get_emoji_loader(emoji_code);
            bird.setup_graphics(sprite_render, &mut commands, &state);
        }
    }
}
