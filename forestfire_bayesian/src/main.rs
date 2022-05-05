// Global imports (needed for the simulation to run)
use crate::model::forest::Forest;
use crate::model::forest::Tree;

mod model;

use krabmaga::{
    argmin::prelude::*,
    argmin::prelude::Error,
    argmin::solver::linesearch::MoreThuenteLineSearch,
    argmin::solver::quasinewton::LBFGS,
    finitediff::FiniteDiff,
    explore::bayesian_opt::*,
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
    statrs::statistics::Distribution,
};

use {
    krabmaga::engine::schedule::*, krabmaga::*, krabmaga::Info, krabmaga::ProgressBar,
    std::time::Duration,
};

use krabmaga::{
    engine::{schedule::Schedule, state::State},
    rand,
    rand::Rng,
    *,
    rand::prelude::*
};


pub const ITERATIONS: usize = 10;
pub const INIT_ELEMENTS: usize = 10;
pub const BATCH_SIZE: usize = 200;

/* pub static STEP: u64 = 10;
pub static WIDTH: i32 = 6400;
pub static HEIGHT: i32 = 6400;
pub const DENSITY: f64 = 0.7; */

lazy_static! {
    pub static ref DATA: Vec<f32> = {
        let mut rdr = Reader::from_path("data/data.csv").unwrap();

        let mut x: Vec<f32> = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            let y: f32 = record[0].parse().unwrap();
            x.push(y);
        }
        x
    };
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(1));
}



fn main() {

    let (x, y) = bayesian_opt!(
        init_population,
        costly_function,
        acquisition_function,
        get_points,
        check_domain,
        ITERATIONS,
    );

    println!("---\nFinal res: Point {:?}, val {y}", x);
}

fn init_population() -> (Vec<Vec<f64>>, Vec<f64>) {
    let mut x_init: Vec<Vec<f64>> = Vec::with_capacity(INIT_ELEMENTS);
    let mut y_init: Vec<f64> = Vec::with_capacity(INIT_ELEMENTS);

    let mut rng = RNG.lock().unwrap();

    for i in 0..INIT_ELEMENTS {
        let density = rng.gen_range(0.01..=1.0_f64); // forest density

        let x = vec![density];
        x_init.push(x);
        y_init.push(costly_function(&x_init[i]));
    }

    (x_init, y_init)
}

fn costly_function(x: &Vec<f64>) -> f64 {
    let density = x[0] as f64;
    let n_step = 500;
    let reps = 3;
    let dim: (i32, i32) = (200, 200);
    let mut steps_tot = 0;
    println!("Point inserted: {:?}", &x);
    
    let mut forest = Forest::new(dim, density);     

    for _ in 0..reps {
        let mut schedule = Schedule::new();
        forest.init(&mut schedule);
        for i in 0..n_step {
            schedule.step(&mut forest);
            if forest.end_condition(&mut schedule) {
                break;
            }
        }

        steps_tot += forest.step;
    }

    println!("AVG steps {}", steps_tot as f64 / reps as f64 );
    return 1. / (steps_tot as f64 / reps as f64)

}

fn get_points(
    x: &Vec<Vec<f64>>
) -> Vec<Vec<f64>> {
    let mut rng = RNG.lock().unwrap();

    let mut trial_x: Vec<Vec<f64>> = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| {
            let density = rng.gen_range(0.1..=1.0_f64); // density
            vec![density]
        })
        .collect();

    trial_x
}

///Expected Improvement algorithm
pub fn acquisition_function(
    gauss_pr: &GaussianProcess<Gaussian, ConstantPrior>,
    x_new: &Vec<f64>,
    x_init: &Vec<Vec<f64>>,
) -> f64 {
    let mean_y_new: f64;
    let mut sigma_y_new: f64;

    mean_y_new = gauss_pr.predict(x_new);
    sigma_y_new = gauss_pr.predict_variance(x_new);
    sigma_y_new = sigma_y_new.sqrt();
    if sigma_y_new == 0. {
        return 0.;
    }

    let mut mean_y: Vec<f64> = Vec::with_capacity(x_init.len());
    for x in x_init {
        mean_y.push(gauss_pr.predict(x));
    }

    // let mean_y_max = mean_y.iter().max().expect("Something goes wrong, no input variables");
    let mut mean_y_max = f64::MIN;
    for m_y in &mean_y {
        if *m_y > mean_y_max {
            mean_y_max = *m_y;
        }
    }

    let z = (mean_y_new - mean_y_max) / sigma_y_new;
    let normal = Normal::new(0.0, 1.0).unwrap();
    let z_cfd = normal.cdf(z);
    let z_pdf = normal.pdf(z);
    (mean_y_new - mean_y_max) * z_cfd + sigma_y_new * z_pdf
}

pub fn check_domain(new_x: &mut Vec<f64>) {
    let density = &mut new_x[0];
    if *density < 0. {
        *density = 0.;
    } else if *density > 1. {
        *density = 1.;
    }

}
