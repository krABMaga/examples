use crate::model::state::Flocker;
use mpi::traits::*;
mod model;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::simulate,
    rust_ab::Info, rust_ab::ProgressBar, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::vis_state::VisState, rust_ab::bevy::prelude::Color,
    rust_ab::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static WIDTH: f32 = 200.;
pub static HEIGHT: f32 = 200.;
pub static NUM_AGENTS: u32 = 100;

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    // pub static STEP: u64 = 10;
    // let state = Flocker::new();
    // simulate!(STEP, state, 1, Info::Verbose);

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);

    let mut x;
    if world.rank() == root_rank {
        x = 2_u64.pow(10);
        println!("Root broadcasting value: {}.", x);
    } else {
        x = 0_u64;
    }
    root_process.broadcast_into(&mut x);
    println!("Rank {} received value: {}.", world.rank(), x);
    assert_eq!(x, 1024);
    println!();

    let mut a;
    let n = 4;
    if world.rank() == root_rank {
        a = (1..).map(|i| 2_u64.pow(i)).take(n).collect::<Vec<_>>();
        println!("Root broadcasting value: {:?}.", &a[..]);
    } else {
        a = std::iter::repeat(0_u64).take(n).collect::<Vec<_>>();
    }
    root_process.broadcast_into(&mut a[..]);
    println!("Rank {} received value: {:?}.", world.rank(), &a[..]);
    assert_eq!(&a[..], &[2, 4, 8, 16]);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    let state = Flocker::new();
    Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(WIDTH as f32, HEIGHT as f32)
        .with_background_color(Color::rgb(0., 0., 0.))
        .with_name("Flockers")
        .start::<VisState, Flocker>(VisState, state);
}
