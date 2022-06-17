use krabmaga::engine::{
    fields::dense_number_grid_2d::DenseNumberGrid2D, fields::dense_object_grid_2d::DenseGrid2D,
    fields::field::Field, location::Int2D, schedule::Schedule, state::State,
};

use krabmaga::*;

use super::sheep::Sheep;
use super::wolf::Wolf;
use crate::{FULL_GROWN, GAIN_ENERGY_SHEEP, GAIN_ENERGY_WOLF, SHEEP_REPR, WOLF_REPR};
use core::fmt;
use hashbrown::HashSet;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::rand;
use krabmaga::rand::Rng;
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
    pub sheep_grid: DenseGrid2D<Sheep>,
    pub grass_field: DenseNumberGrid2D<u16>,
    pub step: u64,
    pub next_id: u32,
    pub new_sheep: Vec<Sheep>,
    pub new_wolves: Vec<Wolf>,
    pub killed_sheep: HashSet<Sheep>,
    pub initial_animals: (u32, u32),
}

impl WsgState {
    pub fn new(dim: (i32, i32), initial_animals: (u32, u32)) -> WsgState {
        WsgState {
            dim,
            wolves_grid: DenseGrid2D::new(dim.0, dim.1),
            sheep_grid: DenseGrid2D::new(dim.0, dim.1),
            grass_field: DenseNumberGrid2D::new(dim.0, dim.1),
            step: 0,
            next_id: initial_animals.1 + initial_animals.0,
            new_sheep: Vec::new(),
            new_wolves: Vec::new(),
            initial_animals,
            killed_sheep: HashSet::new(),
        }
    }
}

impl State for WsgState {
    fn reset(&mut self) {
        self.step = 0;
        self.wolves_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.sheep_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.grass_field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
        self.next_id = self.initial_animals.0 + self.initial_animals.1;
        self.new_sheep = Vec::new();
        self.new_wolves = Vec::new();
        self.initial_animals = (self.initial_animals.0, self.initial_animals.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();
        let s = format!("Also known as Wolf Sheep predation, it is the simulation implemented to introduce \"dynamic scheduling\"
                        feature into the krabmaga framework, because it was the first model with the concepts of \"death\" and \"birth\":
                        there is an ecosystem that involves animals into their life-cycle.");
        description!(s);

        generate_grass(self);
        generate_wolves(self, schedule);
        generate_sheep(self, schedule);

        addplot!(
            String::from("Agents"),
            String::from("Steps"),
            String::from("Number of agents"),
            true
        );

        addplot!(
            String::from("Dead/Born"),
            String::from("Steps"),
            String::from("Number of agents"),
            true
        );
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
        self.sheep_grid.lazy_update();
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
        self.new_sheep.clear();
        self.new_wolves.clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        for sheep in self.new_sheep.iter() {
            schedule.schedule_repeating(Box::new(*sheep), schedule.time + 1.0, 0);
        }

        for wolf in self.new_wolves.iter() {
            schedule.schedule_repeating(Box::new(*wolf), schedule.time + 1.0, 1);
        }

        for sheep in self.killed_sheep.iter() {
            schedule.dequeue(Box::new(*sheep), sheep.id);
        }

        let agents = schedule.get_all_events();
        let mut num_sheep: f32 = 0.;
        let mut num_wolves: f32 = 0.;

        for n in agents {
            if let Some(_s) = n.downcast_ref::<Sheep>() {
                num_sheep += 1.;
            }
            if let Some(_w) = n.downcast_ref::<Wolf>() {
                num_wolves += 1.;
            }
        }

        plot!(
            String::from("Agents"),
            String::from("Wolfs"),
            schedule.step as f64,
            num_wolves as f64
        );
        plot!(
            String::from("Agents"),
            String::from("Sheep"),
            schedule.step as f64,
            num_sheep as f64
        );

        plot!(
            String::from("Dead/Born"),
            String::from("Dead Sheep"),
            schedule.step as f64,
            self.killed_sheep.len() as f64
        );
        plot!(
            String::from("Dead/Born"),
            String::from("Born Wolfs"),
            schedule.step as f64,
            self.new_wolves.len() as f64
        );
        plot!(
            String::from("Dead/Born"),
            String::from("Born Sheep"),
            schedule.step as f64,
            self.new_sheep.len() as f64
        );

        self.killed_sheep.clear();
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

fn generate_sheep(state: &mut WsgState, schedule: &mut Schedule) {
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
        state.sheep_grid.set_object_location(sheep, &loc);

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
