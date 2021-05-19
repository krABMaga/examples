use crate::model::ant::Ant;
use crate::model::state::State;
use crate::model::static_objects::StaticObjectType;
use crate::visualization::static_object::StaticObject;
use crate::{
    FOOD_XMAX, FOOD_XMIN, FOOD_YMAX, FOOD_YMIN, HEIGHT, HOME_XMAX, HOME_XMIN, HOME_YMAX, HOME_YMIN,
    NUM_AGENT, WIDTH,
};
use rust_ab::engine::location::Int2D;
use rust_ab::rand;
use rust_ab::visualization::on_state_init::OnStateInit;
use rust_ab::bevy::prelude::{Commands, ResMut};
use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;
use rust_ab::visualization::renderable::{SpriteType, Render};
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::field::number_grid_2d::BatchRender;
use rust_ab::rand::Rng;
use rust_ab::engine::schedule::Schedule;

pub struct VisState;

impl OnStateInit<Ant> for VisState {
    fn on_init(&self, mut commands: Commands, mut sprite_render_factory: SpriteFactoryResource, mut state: ResMut<State>, mut schedule: ResMut<Schedule<Ant>>, mut sim: ResMut<SimulationDescriptor>) {
        Self::generate_pheromone_visuals(&state, &mut sprite_render_factory, &mut commands, &mut sim);
        Self::generate_nest(&state, &mut sprite_render_factory, &mut commands);
        Self::generate_food(&state, &mut sprite_render_factory, &mut commands);
        Self::generate_obstacles(&state, &mut sprite_render_factory, &mut commands);
        Self::generate_ants(&state, &mut schedule, &mut sprite_render_factory, &mut commands);

        // Update the grids associated to the obstacles and the sites, only once, to write the data from the
        // write buffer to the read buffer
        state.update_obstacles();
        state.update_sites();
    }
}

impl VisState {
    /// Generate the nest site, at a specific location or in a random one within a range.
    fn generate_nest(
        state: &State,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
    ) {
        let mut rng = rand::thread_rng();

        // Generate the nest
        let x: i64 = if HOME_XMIN == HOME_XMAX {
            HOME_XMIN
        } else {
            rng.gen_range(HOME_XMIN..HOME_XMAX)
        };
        let y: i64 = if HOME_YMIN == HOME_YMAX {
            HOME_YMIN
        } else {
            rng.gen_range(HOME_YMIN..HOME_YMAX)
        };

        let position = Int2D { x, y };
        state.set_site(&position, StaticObjectType::HOME);

        let home_vis = StaticObject {
            loc: position,
            emoji_code: String::from("house"),
        };
        let SpriteType::Emoji(emoji_code) = home_vis.sprite();
        let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
        home_vis.setup_graphics(sprite_render, commands, state);
    }

    /// Generate the food site, at a specific location or in a random one within a range.
    fn generate_food(
        state: &State,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
    ) {
        let mut rng = rand::thread_rng();
        // Generate the food resource
        let x: i64 = if FOOD_XMIN == FOOD_XMAX {
            FOOD_XMIN
        } else {
            rng.gen_range(FOOD_XMIN..FOOD_XMAX)
        };
        let y: i64 = if FOOD_YMIN == FOOD_YMAX {
            FOOD_YMIN
        } else {
            rng.gen_range(FOOD_YMIN..FOOD_YMAX)
        };

        let position = Int2D { x, y };
        state.set_site(&position, StaticObjectType::FOOD);

        let food_vis = StaticObject {
            loc: position,
            emoji_code: String::from("candy"),
        };
        let SpriteType::Emoji(emoji_code) = food_vis.sprite();
        let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
        food_vis.setup_graphics(sprite_render, commands, state);
    }

    /// Generate two obstacles, in the form of ellipses made of dense grid cells.
    fn generate_obstacles(
        state: &State,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
    ) {
        /* General formula to calculate an ellipsis, used to draw obstacles.
           x and y define a specific cell
           horizontal and vertical define the ellipsis position (bottom left: 0,0)
           size defines the ellipsis' size (smaller value = bigger ellipsis)
        */
        let ellipsis = |x: f32, y: f32, horizontal: f32, vertical: f32, size: f32| -> bool {
            ((x - horizontal) * size + (y - vertical) * size)
                * ((x - horizontal) * size + (y - vertical) * size)
                / 36.
                + ((x - horizontal) * size - (y - vertical) * size)
                * ((x - horizontal) * size - (y - vertical) * size)
                / 1024.
                <= 1.
        };
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                // Good obstacle placement for 500x500 simulations
                //if ellipsis(i as f32, j as f32, 300., 345., 0.407)
                //    || ellipsis(i as f32, j as f32, 190., 155., 0.407)
                if ellipsis(i as f32, j as f32, 100., 145., 0.407)
                    || ellipsis(i as f32, j as f32, 90., 55., 0.407)
                {
                    let position = Int2D { x: i, y: j };
                    let obstacle_vis = StaticObject {
                        loc: position,
                        emoji_code: String::from("no_entry_sign"),
                    };
                    state.set_obstacle(&position);

                    let SpriteType::Emoji(emoji_code) = obstacle_vis.sprite();
                    let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
                    obstacle_vis.setup_graphics(sprite_render, commands, state);
                }
            }
        }
    }

    /// Generate our ant agents, by creating them in the nest.
    fn generate_ants(
        state: &State,
        schedule: &mut Schedule<Ant>,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
    ) {
        for ant_id in 0..NUM_AGENT {
            let x = (HOME_XMAX + HOME_XMIN) / 2;
            let y = (HOME_YMAX + HOME_YMIN) / 2;
            let loc = Int2D { x, y };
            // Generate the ant with an initial reward of 1, so that it starts spreading home pheromones
            // around the nest, the initial spawn point.
            let mut ant = Ant::new(ant_id, loc, false, 1.);
            state.set_ant_location(&mut ant, &loc);
            schedule.schedule_repeating(ant, 0., 0);

            let SpriteType::Emoji(emoji_code) = ant.sprite();
            let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);
            ant.setup_graphics(sprite_render, commands, state);
        }
    }

    fn generate_pheromone_visuals(
        state: &State,
        sprite_render_factory: &mut SpriteFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state.to_home_grid.render(&mut *sprite_render_factory, commands, sim);
        state.to_food_grid.render(&mut *sprite_render_factory, commands, sim);
    }
}
