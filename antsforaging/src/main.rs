// Global imports, required in all cases
use crate::model::ant::Ant;
use crate::model::state::State;
use rust_ab::engine::schedule::Schedule;

pub mod model;

// Constants
pub const WIDTH: i64 = 200;
pub const HEIGHT: i64 = 200;
pub const NUM_AGENT: u128 = 1000;
pub const EVAPORATION: f64 = 0.999;
pub const STEP: u128 = 1000;
// Nest coordinate range
pub const HOME_XMIN: i64 = 175;
pub const HOME_XMAX: i64 = 175;
pub const HOME_YMIN: i64 = 175;
pub const HOME_YMAX: i64 = 175;
// Food coordinate range
pub const FOOD_XMIN: i64 = 25;
pub const FOOD_XMAX: i64 = 25;
pub const FOOD_YMIN: i64 = 25;
pub const FOOD_YMAX: i64 = 25;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    rust_ab::visualization::visualization::Visualization,
    rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem,
    rust_ab::visualization::field::number_grid_2d::BatchRender,
    crate::visualization::vis_state::VisState,
    crate::model::to_food_grid::ToFoodGrid,
    crate::model::to_home_grid::ToHomeGrid,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
pub mod visualization;

// Main used when a visualization feature is applied
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = State::new(WIDTH, HEIGHT);
    let schedule: Schedule<Ant> = Schedule::new();

    let mut app = Visualization::default()
        .with_background_color(Color::rgb(255.,255.,255.))
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_window_dimensions(800.,800.)
        .setup::<Ant, VisState>(VisState, state, schedule);
    app.add_system(ToHomeGrid::batch_render.system())
        .add_system(ToFoodGrid::batch_render.system());
    app.run()
}

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::rand,
    rust_ab::engine::location::Int2D,
    rust_ab::rand::Rng,
    crate::model::static_objects::StaticObjectType,
    rust_ab::simulate
};

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let mut schedule: Schedule<Ant> = Schedule::new();
    let mut state = State::new(WIDTH, HEIGHT);

    generate_nest(&mut state);
    generate_food(&mut state);
    generate_obstacles(&mut state);
    generate_ants(&mut state, &mut schedule);

    state.update_obstacles();
    state.update_sites();

    simulate!(STEP, schedule, Ant, state);

    /*
        let mut food_found_at = None;
        let mut food_delivered_at = None;

    
     for step in 1..STEP {
        if step % 100 == 0 {
            println!("Milestone {}", step);
        }

        schedule.step(&mut state);

        if food_found_at.is_none() {
            let x = state.food_source_found.read().unwrap();
            if *x {
                food_found_at = Some(step);
                println!("Food source found for the first time at step {}!", step);
            }
        }

        if food_delivered_at.is_none() {
            let x = state.food_returned_home.read().unwrap();
            if *x {
                food_delivered_at = Some(step);
                println!("Food delivered for the first time at step {}!", step);
                break;
            }
        }
    }

    if let (Some(found_at_step), Some(delivered_at_step)) = (food_found_at, food_delivered_at) {
        println!("The path created by ants from the nest to the food takes {} steps.", delivered_at_step - found_at_step);
    } */

}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
/// Generate the nest site, at a specific location or in a random one within a range.
fn generate_nest(state: &mut State) {
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
    state.set_site(&Int2D { x, y }, StaticObjectType::HOME);
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
/// Generate the food site, at a specific location or in a random one within a range.
fn generate_food(state: &mut State) {
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

    state.set_site(&Int2D { x, y }, StaticObjectType::FOOD);
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
/// Generate two obstacles, in the form of ellipses made of dense grid cells.
fn generate_obstacles(state: &mut State) {
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
            if ellipsis(i as f32, j as f32, 100., 145., 0.407)
                || ellipsis(i as f32, j as f32, 90., 55., 0.407)
            {
                state.set_obstacle(&Int2D { x: i, y: j });
            }
        }
    }
}

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
/// Generate our ant agents, by creating them in the nest.
fn generate_ants(state: &mut State, schedule: &mut Schedule<Ant>) {
    for ant_id in 0..NUM_AGENT {
        let x = (HOME_XMAX + HOME_XMIN) / 2;
        let y = (HOME_YMAX + HOME_YMIN) / 2;
        let loc = Int2D { x, y };
        // Generate the ant with an initial reward of 1, so that it starts spreading home pheromones
        // around the nest, the initial spawn point.
        let mut ant = Ant::new(ant_id, loc, false, 1.);
        state.set_ant_location(&mut ant, &loc);
        schedule.schedule_repeating(ant, 0., 0);
    }
}
