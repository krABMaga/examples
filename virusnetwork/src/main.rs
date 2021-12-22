extern crate rust_ab;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::*, rust_ab::engine::state::State, rust_ab::simulate, rust_ab::Info,
    rust_ab::ProgressBar, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem, rust_ab::visualization::fields::network::NetworkRender,
    rust_ab::visualization::visualization::Visualization,
};

use model::state::EpidemicNetworkState;
mod model;

static DISCRETIZATION: f32 = 10.0 / 1.5;
static TOROIDAL: bool = false;
///Initial infected nodes
pub static INITIAL_INFECTED_PROB: f64 = 0.01;
pub static INIT_EDGES: usize = 2;
pub static VIRUS_SPREAD_CHANCE: f64 = 0.3;
pub static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
pub static RECOVERY_CHANCE: f64 = 0.30;
pub static GAIN_RESISTANCE_CHANCE: f64 = 0.20;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let step: u64 = 10;
    let dim: (f32, f32) = (100., 100.);
    let num_nodes = 100;
    let epidemic_network = EpidemicNetworkState::new(dim, num_nodes, DISCRETIZATION, TOROIDAL);

    simulate!(step, epidemic_network, 1, Info::Normal);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let dim: (f32, f32) = (500., 500.);
    let num_nodes = 100;
    let epidemic_network = EpidemicNetworkState::new(dim, num_nodes, DISCRETIZATION, TOROIDAL);

    let mut app = Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::rgb(255., 255., 255.))
        .setup::<VisState, EpidemicNetworkState>(VisState, epidemic_network);

    app.add_system(EpidemicNetworkState::render.system());
    app.run();
}
