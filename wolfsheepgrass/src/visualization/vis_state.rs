use crate::{model::{animals::Animal, grass::FULL_GROWN, state::State}, GAIN_ENERGY, INIT_ENERGY, WOLF_REPR, SHEEP_REPR};

use crate::{HEIGHT, NUM_SHEEPS, NUM_WOLVES, WIDTH};

use rust_ab::engine::location::Int2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::rand;
use rust_ab::rand::Rng;
use rust_ab::visualization::field::number_grid_2d::BatchRender;
use rust_ab::visualization::on_state_init::OnStateInit;
use rust_ab::visualization::renderable::{Render, SpriteType};
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;
use rust_ab::{
    bevy::prelude::{Commands, ResMut},
    rand::prelude::ThreadRng,
};

pub struct VisState;

impl OnStateInit<Animal> for VisState {
    fn on_init(
        &self,
        mut commands: Commands,
        mut sprite_render_factory: SpriteFactoryResource,
        mut state: ResMut<State>,
        mut schedule: ResMut<Schedule<Animal>>,
        mut sim: ResMut<SimulationDescriptor>,
    ) {
        let mut rng = rand::thread_rng();

        Self::generate_grass(
            &state,
            &mut sprite_render_factory,
            &mut commands,
            &mut sim,
            &mut rng,
        );

        Self::generate_wolves(
            &state,
            &mut schedule,
            &mut sprite_render_factory,
            &mut commands,
            &mut rng,
        );

        Self::generate_sheeps(
            &state,
            &mut schedule,
            &mut sprite_render_factory,
            &mut commands,
            &mut rng,
        );

        // Update the grids associated to the obstacles and the sites, only once, to write the data from the
        // write buffer to the read buffer
    }
}

impl VisState {
    fn generate_grass(
        state: &State,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
        rng: &mut ThreadRng,
    ) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let grass_init_value = rng.gen_range(0..FULL_GROWN + 1);
                state.set_grass_at_location(&Int2D { x, y }, grass_init_value);
            }
        }

        state
            .grass_field
            .render(&mut *sprite_render_factory, commands, sim);
    }

    fn generate_wolves(
        state: &State,
        schedule: &mut Schedule<Animal>,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) {
        for wolf_id in 0..NUM_WOLVES {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            let loc = Int2D { x, y };

            let mut wolf = Animal::new_wolf(wolf_id, loc, INIT_ENERGY, GAIN_ENERGY, WOLF_REPR);
            state.set_wolf_location(&mut wolf, &loc);
            // Sheep have an higher ordering than wolves. This is so that if a wolf kills one, in the next step
            // the attacked sheep will immediately notice and die, instead of noticing after two steps.
            schedule.schedule_repeating(wolf, 0., 1);

            let SpriteType::Emoji(emoji_code) = wolf.sprite();
            let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
            wolf.setup_graphics(sprite_render, commands, state);
        }
    }

    fn generate_sheeps(
        state: &State,
        schedule: &mut Schedule<Animal>,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
        rng: &mut ThreadRng,
    ) {
        for sheep_id in 0..NUM_SHEEPS {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            let loc = Int2D { x, y };

            let mut sheep = Animal::new_sheep(
                sheep_id + NUM_WOLVES + 1,
                loc,
                INIT_ENERGY,
                GAIN_ENERGY,
                SHEEP_REPR,
            );
            state.set_sheep_location(&sheep, &loc);
            schedule.schedule_repeating(sheep, 0., 0);

            let SpriteType::Emoji(emoji_code) = sheep.sprite();
            let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
            sheep.setup_graphics(sprite_render, commands, state);
        }
    }
}
