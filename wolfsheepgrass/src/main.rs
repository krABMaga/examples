use rust_ab::engine::location::Int2D;
use crate::model::state::State;
use model::{animals::Animal, grass::{FULL_GROWN,  GrassField}};

mod model;
use rust_ab::engine::schedule::Schedule;
use rust_ab::rand::Rng;
use rust_ab::rand;
use rayon::prelude::*;


pub const ENERGY_CONSUME: f64  = 5.0;
pub const NUM_WOLVES: u128 = 1;
pub const NUM_SHEEPS: u128 = 1;


pub const INIT_ENERGY: f64 = 100.0;
pub const GAIN_ENERGY: f64 = 10.0;

pub const WOLF_REPR: f64 = 0.1;

pub const WIDTH: i64 = 50;
pub const HEIGHT: i64 = 50;
pub const STEP: u128 = 100;

//----------------------------------------------------------------
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main(){
    
    let mut state = State::new(WIDTH, HEIGHT, Schedule::new());

    generate_food(&mut state);
    generate_wolves(&mut state);
    generate_sheeps(&mut state);

    let scheduler = &state.scheduler;
    for step in 1..STEP{
        if step%100 == 0 {
            println!("Milestone {}", step);
        }
        
        state.step = step;
        scheduler.step(&state);
        
    }
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_food(state: &mut State) -> () {

    (0.. HEIGHT).into_par_iter().for_each(|x| {
        (0.. WIDTH).into_par_iter().for_each(|y| {
            let mut rng = rand::thread_rng();
            let grass_init_value = rng.gen_range(0..FULL_GROWN+1);
            state.set_grass_at_location(&Int2D{x,y}, grass_init_value);
        })
    });

}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_sheeps(state: &mut State) -> () {
    let schedule = &state.scheduler;
    let mut rng = rand::thread_rng();
    
    for id in 0..NUM_SHEEPS {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let loc = Int2D { x, y };
 
        let mut sheep = Animal::new_sheep(id + NUM_WOLVES, loc, INIT_ENERGY, GAIN_ENERGY, WOLF_REPR);
       // println!("Sheep initial loc: {} {} \n------------", loc.x, loc.y);
        state.set_sheep_location(&mut sheep, &loc);
        schedule.schedule_repeating(sheep, 0., 0);
    }
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn generate_wolves(state: &mut State) -> () {

    let schedule = &state.scheduler;
    let mut rng = rand::thread_rng();
    for id in 0..NUM_WOLVES {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        let loc = Int2D { x, y };
 
        let mut wolf = Animal::new_wolf(id, loc, INIT_ENERGY, GAIN_ENERGY, WOLF_REPR);
        state.set_wolf_location(&mut wolf, &loc);
        schedule.schedule_repeating(wolf, 0., 0);
    }
}

//----------------------------------------------------------------

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;


// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    rust_ab::visualization::visualization::Visualization,
    rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem,
    rust_ab::visualization::field::number_grid_2d::BatchRender,
    crate::visualization::vis_state::VisState,
};

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    
    let state = State::new(WIDTH, HEIGHT, Schedule::new());
    let scheduler = state.scheduler;
    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255.,255.,255.))
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(600.,600.)
        .setup::<Animal, VisState>(VisState, state, scheduler);
    app.add_system(GrassField::batch_render.system());
    app.run()
}