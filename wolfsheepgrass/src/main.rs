use crate::model::state::State;
use model::{
    animals::Animal,
    grass::{GrassField, FULL_GROWN},
};
use rust_ab::engine::location::Int2D;

mod model;
use rayon::prelude::*;
use rust_ab::engine::schedule::Schedule;
use rust_ab::rand;
use rust_ab::rand::Rng;

pub const ENERGY_CONSUME: f64 = 5.0;
pub const NUM_WOLVES: u128 = 1;
pub const NUM_SHEEPS: u128 = 1;

pub const INIT_ENERGY: f64 = 100.0;
pub const GAIN_ENERGY: f64 = 10.0;

pub const SHEEP_REPR: f64 = 0.1;
pub const WOLF_REPR: f64 = 0.01;

pub const WIDTH: i64 = 50;
pub const HEIGHT: i64 = 50;
pub const STEP: u128 = 100;

//----------------------------------------------------------------
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let mut state = State::new(WIDTH, HEIGHT);
    let mut schedule = Schedule::<Animal>::new();

    generate_food(&mut state);
    generate_wolves(&mut state, &mut schedule);
    generate_sheeps(&mut state, &mut schedule);

    for step in 1..STEP {
        if step % 100 == 0 {
            println!("Milestone {}", step);
        }

        schedule.step(&mut state);
    }
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_food(state: &mut State) -> () {
    (0..HEIGHT).into_par_iter().for_each(|x| {
        (0..WIDTH).into_par_iter().for_each(|y| {
            let mut rng = rand::thread_rng();
            let grass_init_value = rng.gen_range(0..FULL_GROWN + 1);
            state.set_grass_at_location(&Int2D { x, y }, grass_init_value);
        })
    });
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_sheeps(state: &mut State, schedule: &mut Schedule<Animal>) -> () {
    let mut rng = rand::thread_rng();

    for id in 0..NUM_SHEEPS {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let loc = Int2D { x, y };

        let mut sheep =
            Animal::new_sheep(id + NUM_WOLVES, loc, INIT_ENERGY, GAIN_ENERGY, SHEEP_REPR);
        // println!("Sheep initial loc: {} {} \n------------", loc.x, loc.y);
        state.set_sheep_location(&mut sheep, &loc);
        schedule.schedule_repeating(sheep, 0., 0);
    }
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_wolves(state: &mut State, schedule: &mut Schedule<Animal>) -> () {
    let mut rng = rand::thread_rng();
    for id in 0..NUM_WOLVES {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let loc = Int2D { x, y };

        let mut wolf = Animal::new_wolf(id, loc, INIT_ENERGY, GAIN_ENERGY, WOLF_REPR);
        state.set_wolf_location(&mut wolf, &loc);
        // Wolves have a higher priority than sheep. This lets the Schedule process wolves first, so
        // that a wolf killing a sheep is taken into account in the same step.
        schedule.schedule_repeating(wolf, 0., 1);
    }
}

//----------------------------------------------------------------

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem, rust_ab::visualization::field::number_grid_2d::BatchRender,
    rust_ab::visualization::visualization::Visualization,
};

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = State::new(WIDTH, HEIGHT);
    let scheduler = Schedule::new();
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255., 255., 255.))
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(600., 600.)
        .setup::<Animal, VisState>(VisState, state, scheduler);
    app.add_system(GrassField::batch_render.system());
    app.run()
}
