mod model;
use crate::model::state::Flocker;



use {rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::*};

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;
pub static NEIGHBOROOD_RADIUS: f32 = 10.;

#[cfg(feature="explore")]
fn main() {
    let universe = mpi::initialize().unwrap();
    let step = 100;
    let dim = (200., 200.);
    let num_agents = 10000;    
    let state = Flocker::new(dim,num_agents,universe);

    simulate!(step,state,1,Info::Normal);
}

