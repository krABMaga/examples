use rand::distributions::weighted::WeightedIndex;

use krabmaga::{
    argmin::prelude::Error,
    argmin::prelude::*,
    argmin::solver::linesearch::MoreThuenteLineSearch,
    argmin::solver::quasinewton::LBFGS,
    explore::bayesian_opt::*,
    finitediff::FiniteDiff,
    friedrich::gaussian_process::GaussianProcess,
    friedrich::kernel::Gaussian,
    friedrich::prior::ConstantPrior,
    statrs::distribution::{Continuous, ContinuousCDF, Normal},
    statrs::statistics::Distribution,
};

use krabmaga::{
    engine::{schedule::Schedule, state::State},
    rand,
    rand::Rng,
    *,
};

use rayon::prelude::*;

use rand::prelude::*;

use model::state::EpidemicNetworkState;
// use std::cmp::Ordering::Equal;
use std::io::Write;
mod model;

// generic model parameters
pub static INIT_EDGES: usize = 1;
pub const NUM_NODES: u32 = 5_000;
pub static DESIRED_RT: f32 = 2.;
// pub static INITIAL_INFECTED: f32 = 0.01;

// compute the rt at 30 steps (30 days)
pub const STEP: u64 = 37;
pub const REPETITION: usize = 20;

pub const ITERATIONS: usize = 100;
pub const INIT_ELEMENTS: usize = 10;
pub const VAR_NUMS: usize = 4;
pub const BATCH_SIZE: usize = 200;

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
        let spread = rng.gen_range(0.0..=1.0_f64); // spread chance
        let r1 = rng.gen_range(0.0..=1.0_f64); // recovery chance
        let r2 = rng.gen_range(0.0..=1.0_f64); // recovery chance
        let day = rng.gen_range(0..=31) as f64; // day

        let x = vec![spread, r1, r2, day];
        x_init.push(x);
        y_init.push(costly_function(&x_init[i]));
    }

    (x_init, y_init)
}

fn costly_function(x: &Vec<f64>) -> f64 {
    let spread = x[0] as f32;
    let recovery = x[1] as f32;
    let spread2 = x[2] as f32;
    let day = x[3] as u64;

    let mut avg_results: Vec<f32> = vec![0.0; 31];
    println!("Point inserted: {:?}", &x);

    let mut vec_winfected: Vec<Vec<f32>> = Vec::new();
    (0..REPETITION)
        .into_par_iter()
        .map(|i| {
            // println!("Running simulation {}...", i);
            let mut state = EpidemicNetworkState::new(spread, recovery, spread2, day, i);
            simulate!(STEP, &mut state, 1, Info::Verbose);
            state.weekly_infected
        })
        .collect_into_vec(&mut vec_winfected);

    for weekly_infected in &vec_winfected {
        for j in 0..31 {
            avg_results[j] += weekly_infected[j] / NUM_NODES as f32;
        }
    }

    //parallelize it
    // for i in 0..REPETITION as usize {
    //     let mut state = EpidemicNetworkState::new(spread, recovery, spread2, day, i);
    //     simulate!(STEP, &mut state, 1, Info::Verbose);
    //     for j in 0..31 {
    //         avg_results[j] += state.weekly_infected[j] / NUM_NODES as f32;
    //     }
    // }

    for j in 0..31 {
        avg_results[j] /= REPETITION as f32;
    }

    let mut ind_error = 0.;
    let mut sum = 0.;
    let alpha: f32 = 0.15;
    for k in 0..31 {
        let weight = 1. / (alpha * (1. - alpha).powf(k as f32));
        ind_error += weight as f32 * ((DATA[k] - avg_results[k]) / DATA[k]).powf(2.);
        sum += weight as f32;
    }
    ind_error = (ind_error / (sum * 31.)).sqrt();

    println!("Avg_error: {} ", ind_error);
    ind_error as f64
}

fn get_points(x: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut rng = RNG.lock().unwrap();

    let mut trial_x: Vec<Vec<f64>> = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| {
            let spread = rng.gen_range(0.0..=1.0_f64); // spread chance
            let r1 = rng.gen_range(0.0..=1.0_f64); // recovery chance
            let r2 = rng.gen_range(0.0..=1.0_f64); // recovery chance
            let day = rng.gen_range(17..=31) as f64; // day
            vec![spread, r1, r2, day]
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
    let recovery = &mut new_x[0];
    if *recovery < 0. {
        *recovery = 0.;
    } else if *recovery > 1. {
        *recovery = 1.;
    }

    let spread = &mut new_x[1];
    if *spread < 0. {
        *spread = 0.;
    } else if *spread > 1. {
        *spread = 1.;
    }

    let spread2 = &mut new_x[2];
    if *spread2 < 0. {
        *spread2 = 0.;
    } else if *spread2 > 1. {
        *spread2 = 1.;
    }

    let day = &mut new_x[3];
    if *day < 0. {
        *day = 0.;
    } else if *day > 31. {
        *day = 31.;
    } else {
        *day = day.ceil()
    }
}
