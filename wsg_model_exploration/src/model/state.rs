use rust_ab::engine::{
    fields::dense_number_grid_2d::DenseNumberGrid2D, fields::dense_object_grid_2d::DenseGrid2D,
    fields::field::Field, location::Int2D, schedule::Schedule, state::State,
};

use super::sheep::Sheep;
use super::wolf::Wolf;
use crate::{FULL_GROWN, GAIN_ENERGY_SHEEP, GAIN_ENERGY_WOLF, SHEEP_REPR, WOLF_REPR};
use core::fmt;
use rust_ab::engine::fields::grid_option::GridOption;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;
pub use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    u32,
};

#[derive(Clone, Copy, PartialEq)]
pub enum LifeState {
    Alive,
    Dead,
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LifeState::Alive => write!(f, "Alive"),
            LifeState::Dead => write!(f, "Dead"),
        }
    }
}

pub struct WsgState {
    pub dim: (i32, i32),
    pub wolves_grid: DenseGrid2D<Wolf>,
    pub sheeps_grid: DenseGrid2D<Sheep>,
    pub grass_field: DenseNumberGrid2D<u16>,
    pub step: u64,
    pub next_id: Arc<Mutex<u32>>,
    pub eaten_grass: Arc<Mutex<Vec<Int2D>>>,
    pub new_sheeps: Arc<Mutex<Vec<Sheep>>>,
    pub new_wolves: Arc<Mutex<Vec<Wolf>>>,
    //initial.animals.0 SHEEP initial.animals.1 WOLF
    pub initial_animals: (u32, u32),
    pub survived_wolves: u32,
    pub survived_sheeps: u32,
}

impl State for WsgState {
    fn reset(&mut self) {
        self.step = 0;
        self.dim = self.dim;
        self.wolves_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.sheeps_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.grass_field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
        self.next_id = Arc::new(Mutex::new(self.initial_animals.0 + self.initial_animals.1));
        self.eaten_grass = Arc::new(Mutex::new(Vec::new()));
        self.new_sheeps = Arc::new(Mutex::new(Vec::new()));
        self.new_wolves = Arc::new(Mutex::new(Vec::new()));
        self.initial_animals = (self.initial_animals.0, self.initial_animals.1);
        self.survived_wolves = self.initial_animals.1;
        self.survived_sheeps = self.initial_animals.0;
    }

    fn init(&mut self, schedule: &mut Schedule) {
        generate_grass(self);
        generate_wolves(self, schedule);
        generate_sheeps(self, schedule);
    }

    fn update(&mut self, step: u64) {
        if step != 0 {
            self.grass_field.apply_to_all_values(
                |grass| {
                    let growth = *grass;
                    if growth < FULL_GROWN {
                        growth + 1
                    } else {
                        growth
                    }
                },
                GridOption::READWRITE,
            );
        }

        self.grass_field.lazy_update();
        self.sheeps_grid.lazy_update();
        self.wolves_grid.lazy_update();

        self.step = step;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {
        self.new_sheeps.lock().unwrap().clear();
        self.new_wolves.lock().unwrap().clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        self.eaten_grass.lock().unwrap().clear();

        let agents = schedule.get_all_events();
        let mut sheeps = 0;
        let mut wolves = 0;
        for n in agents {
            match n.downcast_ref::<Sheep>() {
                Some(_p) => {
                    sheeps += 1;
                }
                None => {}
            }
            match n.downcast_ref::<Wolf>() {
                Some(_l) => {
                    wolves += 1;
                }
                None => {}
            }
        }

        self.survived_sheeps = sheeps;
        self.survived_wolves = wolves;

        // let mut grasses = 0;
        // for i in 0..self.dim.0 {
        //     for j in 0..self.dim.1 {
        //         match self
        //             .grass_field
        //             .get_value(&Int2D { x: i, y: j })
        //         {
        //             Some(v) => {
        //                 // println!("Grass {:?} has value {:?}", Int2D { x: i, y: j }, v);
        //                 if v == FULL_GROWN {
        //                     grasses += 1;
        //                 }
        //             }
        //             None => {
        //                 //  println!("Grass {:?} not found", Int2D { x: i, y: j });
        //             }
        //         }
        //     }
        // }

        // println!(
        //     "Number of sheeps: {:?} - wolves: {:?} - full growth grasses: {:?} at step {:?}\n",
        //     sheeps, wolves, grasses, schedule.step
        // );
    }
}

fn generate_grass(state: &mut WsgState) {
    (0..state.dim.1).into_iter().for_each(|x| {
        (0..state.dim.0).into_iter().for_each(|y| {
            let mut rng = rand::thread_rng();
            let fully_growth = rng.gen_bool(0.5);
            if fully_growth {
                state
                    .grass_field
                    .set_value_location(FULL_GROWN, &Int2D { x, y });
            } else {
                let grass_init_value = rng.gen_range(0..FULL_GROWN + 1);
                state
                    .grass_field
                    .set_value_location(grass_init_value, &Int2D { x, y });
            }
        })
    });
}

fn generate_sheeps(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();

    for id in 0..state.initial_animals.0 {
        let loc = Int2D {
            x: rng.gen_range(0..state.dim.0),
            y: rng.gen_range(0..state.dim.1),
        };
        let init_energy = rng.gen_range(0..(2 * GAIN_ENERGY_SHEEP as usize));
        let sheep = Sheep::new(
            id + state.initial_animals.1,
            loc,
            init_energy as f64,
            GAIN_ENERGY_SHEEP,
            SHEEP_REPR,
        );
        state.sheeps_grid.set_object_location(sheep, &loc);

        schedule.schedule_repeating(Box::new(sheep), 0., 0);
    }
}

fn generate_wolves(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();
    for id in 0..state.initial_animals.1 {
        let loc = Int2D {
            x: rng.gen_range(0..state.dim.0),
            y: rng.gen_range(0..state.dim.1),
        };
        let init_energy = rng.gen_range(0..(2 * GAIN_ENERGY_WOLF as usize));

        let wolf = Wolf::new(id, loc, init_energy as f64, GAIN_ENERGY_WOLF, WOLF_REPR);
        state.wolves_grid.set_object_location(wolf, &loc);

        // Sheep have an higher ordering than wolves. This is so that if a wolf kills one, in the next step
        // the attacked sheep will immediately notice and die, instead of noticing after two steps.
        schedule.schedule_repeating(Box::new(wolf), 0., 1);
    }
}

impl WsgState {
    pub fn new(dim: (i32, i32), initial_animals: (u32, u32)) -> WsgState {
        WsgState {
            dim: dim,
            wolves_grid: DenseGrid2D::new(dim.0, dim.1),
            sheeps_grid: DenseGrid2D::new(dim.0, dim.1),
            grass_field: DenseNumberGrid2D::new(dim.0, dim.1),
            step: 0,
            next_id: Arc::new(Mutex::new(initial_animals.1 + initial_animals.0)),
            eaten_grass: Arc::new(Mutex::new(Vec::new())),
            new_sheeps: Arc::new(Mutex::new(Vec::new())),
            new_wolves: Arc::new(Mutex::new(Vec::new())),
            initial_animals: initial_animals,
            survived_wolves: initial_animals.1,
            survived_sheeps: initial_animals.0,
        }
    }

    // pub fn new(width: i32,
    //     height: i32,
    //     num_wolves: u32,
    //     num_sheeps: u32,
    //     energy_consume: f64,
    //     full_grown: u16,
    //     gain_energy_sheep: f64,
    //     gain_energy_wolf: f64,
    //     sheep_repr: f64,
    //     wolf_repr: f64) -> WsgState {

    //     WsgState {
    //         wolves_grid: DenseGrid2D::new(width, height),
    //         sheeps_grid: DenseGrid2D::new(width, height),
    //         grass_field: DenseNumberGrid2D::new(width, height),
    //         step: 0,
    //         next_id: Arc::new(Mutex::new(NUM_WOLVES + NUM_SHEEPS)),
    //         eaten_grass: Arc::new(Mutex::new(Vec::new())),
    //         new_sheeps: Arc::new(Mutex::new(Vec::new())),
    //         new_wolves: Arc::new(Mutex::new(Vec::new())),
    //         num: (NUM_WOLVES, NUM_SHEEPS),
    //         survived_wolves: NUM_WOLVES,
    //         survived_sheeps: NUM_SHEEPS,
    //     }
    // }
}