use rust_ab::engine::{
    fields::dense_number_grid_2d::DenseNumberGrid2D, fields::dense_object_grid_2d::DenseGrid2D,
    fields::field::Field, location::Int2D, schedule::Schedule, state::State,
};

use super::sheep::Sheep;
use super::wolf::Wolf;
use crate::{FULL_GROWN, GAIN_ENERGY_SHEEP, GAIN_ENERGY_WOLF, SHEEP_REPR, WOLF_REPR};
use core::fmt;
use hashbrown::HashSet;
use rust_ab::engine::fields::grid_option::GridOption;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;
pub use std::time::Duration;
pub use std::time::Instant;

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
    pub next_id: u32,
    pub new_sheeps: Vec<Sheep>,
    pub new_wolves: Vec<Wolf>,
    pub killed_sheeps: HashSet<Sheep>,
    pub initial_animals: (u32, u32),
}

impl WsgState {
    pub fn new(dim: (i32, i32), initial_animals: (u32, u32)) -> WsgState {
        WsgState {
            dim,
            wolves_grid: DenseGrid2D::new(dim.0, dim.1),
            sheeps_grid: DenseGrid2D::new(dim.0, dim.1),
            grass_field: DenseNumberGrid2D::new(dim.0, dim.1),
            step: 0,
            next_id: initial_animals.1 + initial_animals.0,
            new_sheeps: Vec::new(),
            new_wolves: Vec::new(),
            initial_animals,
            killed_sheeps: HashSet::new(),
        }
    }
}

impl State for WsgState {
    fn reset(&mut self) {
        self.step = 0;
        self.wolves_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.sheeps_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.grass_field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
        self.next_id = self.initial_animals.0 + self.initial_animals.1;
        self.new_sheeps = Vec::new();
        self.new_wolves = Vec::new();
        self.initial_animals = (self.initial_animals.0, self.initial_animals.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let start = Instant::now();

        generate_grass(self);
        generate_wolves(self, schedule);
        generate_sheeps(self, schedule);
        let elapsed = start.elapsed();
        println!("Elapsed {}", elapsed.as_secs_f32());

    
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
        self.new_sheeps.clear();
        self.new_wolves.clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {

        for sheep in self.new_sheeps.iter() {
            schedule.schedule_repeating(Box::new(*sheep), schedule.time + 1.0, 0);
        }

        for wolf in self.new_wolves.iter() {
            schedule.schedule_repeating(Box::new(*wolf), schedule.time + 1.0, 1);
        }

        for sheep in self.killed_sheeps.iter() {
            schedule.dequeue(Box::new(*sheep), sheep.id);
        }

        self.killed_sheeps.clear();
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


