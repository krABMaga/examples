use rust_ab::engine::{
    fields::dense_number_grid_2d::DenseNumberGrid2D, fields::dense_object_grid_2d::DenseGrid2D,
    fields::field::Field, location::Int2D, schedule::Schedule, state::State,
};

use super::sheep::Sheep;
use super::wolf::Wolf;
use crate::{HEIGHT, INITIAL_NUM_SHEEPS, INITIAL_NUM_WOLVES, WIDTH};
use core::fmt;
use rust_ab::engine::fields::grid_option::GridOption;
use rust_ab::hashbrown::HashSet;
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
    pub wolves_grid: DenseGrid2D<Wolf>,
    pub sheeps_grid: DenseGrid2D<Sheep>,
    pub grass_field: DenseNumberGrid2D<u16>,
    pub step: u64,
    pub next_id: Arc<Mutex<u32>>,
    pub eaten_grass: Arc<Mutex<Vec<Int2D>>>,
    pub new_sheeps: Arc<Mutex<Vec<Sheep>>>,
    pub new_wolves: Arc<Mutex<Vec<Wolf>>>,
    pub killed_sheeps: Arc<Mutex<HashSet<Sheep>>>,
    pub survived_wolves: u32,
    pub survived_sheeps: u32,
    pub gain_energy_sheep: f32,
    pub gain_energy_wolf: f32,
    pub sheep_repr: f32,
    pub wolf_repr: f32,
    pub fitness: f32,
    pub full_grown: u16,
}

impl fmt::Display for WsgState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WsgState 
        fitness: {} |
        gain_energy_sheep: {} |
        gain_enery_wolf: {} |
        sheep_repr: {} |
        wolf_repr: {} | 
        full_grown: {}",
            self.fitness,
            self.gain_energy_sheep,
            self.gain_energy_wolf,
            self.sheep_repr,
            self.wolf_repr,
            self.full_grown
        )
    }
}

impl State for WsgState {
    fn reset(&mut self) {
        self.step = 0;
        self.wolves_grid = DenseGrid2D::new(WIDTH, HEIGHT);
        self.sheeps_grid = DenseGrid2D::new(WIDTH, HEIGHT);
        self.grass_field = DenseNumberGrid2D::new(WIDTH, HEIGHT);
        self.next_id = Arc::new(Mutex::new(INITIAL_NUM_SHEEPS + INITIAL_NUM_WOLVES));
        self.eaten_grass = Arc::new(Mutex::new(Vec::new()));
        self.new_sheeps = Arc::new(Mutex::new(Vec::new()));
        self.new_wolves = Arc::new(Mutex::new(Vec::new()));
        self.killed_sheeps = Arc::new(Mutex::new(HashSet::new()));
        //self.initial_animals = (INITIAL_NUM_SHEEPS, INITIAL_NUM_WOLVES);
        self.survived_wolves = INITIAL_NUM_WOLVES;
        self.survived_sheeps = INITIAL_NUM_SHEEPS;
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
                    if growth < self.full_grown {
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

        for sheep in self.new_sheeps.lock().unwrap().iter() {
            schedule.schedule_repeating(Box::new(*sheep), schedule.time + 1.0, 0);
        }

        for wolf in self.new_wolves.lock().unwrap().iter() {
            schedule.schedule_repeating(Box::new(*wolf), schedule.time + 1.0, 1);
        }

        let agents = schedule.get_all_events();
        let mut sheeps = 0;
        let mut wolves = 0;
        for n in agents {
            if let Some(_p) = n.downcast_ref::<Sheep>() {
                sheeps += 1;
            }
            if let Some(_p) = n.downcast_ref::<Wolf>() {
                wolves += 1;
            }
        }

        self.survived_sheeps = sheeps;
        self.survived_wolves = wolves;

        // let mut grasses = 0;
        // for i in 0..WIDTH {
        //     for j in 0..HEIGHT {
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

    fn end_condition(&mut self, schedule: &mut Schedule) -> bool {
        let agents = schedule.get_all_events();
        let mut num_sheeps: f32 = 0.;
        let mut num_wolves: f32 = 0.;

        for n in agents {
            if let Some(_s) = n.downcast_ref::<Sheep>() {
                num_sheeps += 1.;
            }
            if let Some(_w) = n.downcast_ref::<Wolf>() {
                num_wolves += 1.;
            }
        }

        if num_sheeps == 0. || num_wolves == 0. {
            return true;
        }

        false
    }
}

fn generate_grass(state: &mut WsgState) {
    (0..HEIGHT).into_iter().for_each(|x| {
        (0..WIDTH).into_iter().for_each(|y| {
            let mut rng = rand::thread_rng();
            let fully_growth = rng.gen_bool(0.5);
            if fully_growth {
                state
                    .grass_field
                    .set_value_location(state.full_grown, &Int2D { x, y });
            } else {
                let grass_init_value = rng.gen_range(0..state.full_grown + 1);
                state
                    .grass_field
                    .set_value_location(grass_init_value, &Int2D { x, y });
            }
        })
    });
}

fn generate_sheeps(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();

    for id in 0..INITIAL_NUM_SHEEPS {
        let loc = Int2D {
            x: rng.gen_range(0..WIDTH),
            y: rng.gen_range(0..HEIGHT),
        };
        let init_energy = rng.gen_range(0..(2 * state.gain_energy_sheep as usize));
        let sheep = Sheep::new(
            id + INITIAL_NUM_WOLVES,
            loc,
            init_energy as f32,
            state.gain_energy_sheep,
            state.sheep_repr,
        );
        state.sheeps_grid.set_object_location(sheep, &loc);

        schedule.schedule_repeating(Box::new(sheep), 0., 0);
    }
}

fn generate_wolves(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::thread_rng();
    for id in 0..INITIAL_NUM_WOLVES {
        let loc = Int2D {
            x: rng.gen_range(0..WIDTH),
            y: rng.gen_range(0..HEIGHT),
        };
        let init_energy = rng.gen_range(0..(2 * state.gain_energy_wolf as usize));

        let wolf = Wolf::new(
            id,
            loc,
            init_energy as f32,
            state.gain_energy_wolf,
            state.wolf_repr,
        );
        state.wolves_grid.set_object_location(wolf, &loc);

        // Sheep have an higher ordering than wolves. This is so that if a wolf kills one, in the next step
        // the attacked sheep will immediately notice and die, instead of noticing after two steps.
        schedule.schedule_repeating(Box::new(wolf), 0., 1);
    }
}

impl WsgState {
    pub fn new(
        gain_energy_sheep: f32,
        gain_energy_wolf: f32,
        sheep_repr: f32,
        wolf_repr: f32,
        full_grown: u16,
    ) -> WsgState {
        WsgState {
            wolves_grid: DenseGrid2D::new(WIDTH, HEIGHT),
            sheeps_grid: DenseGrid2D::new(WIDTH, HEIGHT),
            grass_field: DenseNumberGrid2D::new(WIDTH, HEIGHT),
            step: 0,
            next_id: Arc::new(Mutex::new(INITIAL_NUM_WOLVES + INITIAL_NUM_SHEEPS)),
            eaten_grass: Arc::new(Mutex::new(Vec::new())),
            new_sheeps: Arc::new(Mutex::new(Vec::new())),
            new_wolves: Arc::new(Mutex::new(Vec::new())),
            survived_wolves: INITIAL_NUM_WOLVES,
            survived_sheeps: INITIAL_NUM_SHEEPS,
            killed_sheeps: Arc::new(Mutex::new(HashSet::new())),
            gain_energy_sheep,
            gain_energy_wolf,
            sheep_repr,
            wolf_repr,
            fitness: 0.,
            full_grown,
        }
    }
}
