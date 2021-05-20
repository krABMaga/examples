// Global imports (needed for the simulation to run)
use crate::model::my_state::MyState;
use rust_ab::engine::schedule::Schedule;
use crate::model::my_agent::MyAgent;
use rust_ab::simulate;
mod model;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    rust_ab::visualization::visualization::Visualization,
    crate::visualization::my_vis_state::MyVisState
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let state = MyState::new();
    let schedule = Schedule::<MyAgent>::new();

    Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(500., 500.)
        .start::<MyAgent, MyVisState>(MyVisState, state, schedule);
}

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    // Initialize the simulation here.
    static STEP: u128 = 50;

    let mut state = MyState::new();
    let mut schedule = Schedule::<MyAgent>::new();

    let my_agent = MyAgent{id: 1};
    // Put the agent in your state
    schedule.schedule_repeating(my_agent, 0., 0);

    simulate!(STEP, schedule, MyAgent, state);

    /* for _ in 0..STEP {
        schedule.step(&mut state);
    }

    println!("The simulation has completed successfully.") */;
}
