// Global imports (needed for the simulation to run)
use crate::model::forest::Forest;
use crate::model::forest::Tree;

mod model;

#[cfg(any(feature = "bayesian"))]
use krabmaga::{engine::schedule::Schedule, engine::state::State, explore::bayesian::*};

use krabmaga::{rand::prelude::*, *};

pub const ITERATIONS: usize = 10;
pub const INIT_ELEMENTS: usize = 4;
pub const BATCH_SIZE: usize = 200;

/* pub static STEP: u64 = 10;
pub static WIDTH: i32 = 6400;
pub static HEIGHT: i32 = 6400;
pub const DENSITY: f64 = 0.7; */

lazy_static! {
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(10));
}

#[cfg(not(any(feature = "bayesian")))]
fn main() {
    let density = 100.;
    let dim: (i32, i32) = (200, 200);
    let mut _forest = Forest::new(dim, density);
    println!("No bayesian feature enabled");
}

#[cfg(any(feature = "bayesian"))]
fn main() {
    let (x, y) = bayesian_search!(init_population, objective, get_points, ITERATIONS);

    println!("---\nFinal res: Point {:?}, val {y}", x);
}

#[cfg(any(feature = "bayesian"))]
fn init_population() -> Vec<Vec<f64>> {
    let mut x_init: Vec<Vec<f64>> = Vec::with_capacity(INIT_ELEMENTS);

    let mut rng = RNG.lock().unwrap();

    for _ in 0..INIT_ELEMENTS {
        let density = rng.gen_range(0.01..=1.0_f64); // forest density
        x_init.push(vec![density]);
    }

    x_init
}

#[cfg(any(feature = "bayesian"))]
fn objective(x: &[f64]) -> f64 {
    let density = x[0];
    let n_step = 500;
    let reps = 3;
    let dim: (i32, i32) = (200, 200);
    let mut steps_tot = 0;

    let mut forest = Forest::new(dim, density);

    for _ in 0..reps {
        let mut schedule = Schedule::new();
        forest.init(&mut schedule);
        for _ in 0..n_step {
            schedule.step(&mut forest);
            if forest.end_condition(&mut schedule) {
                break;
            }
        }

        steps_tot += forest.step;
    }

    println!("AVG steps {}", steps_tot as f64 / reps as f64);

    steps_tot as f64 / reps as f64
}

#[cfg(any(feature = "bayesian"))]
fn get_points(_x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut rng = RNG.lock().unwrap();

    let trial_x: Vec<Vec<f64>> = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| {
            let density = rng.gen_range(0.1..=1.0_f64); // density
            vec![density]
        })
        .collect();

    trial_x
}
