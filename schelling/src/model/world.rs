use crate::model::updater::Updater;
use core::fmt;
use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use rust_ab::engine::location::Int2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::hash::Hash;
use std::hash::Hasher;

use crate::{HEIGHT, NUM_AGENTS, PERC, WIDTH};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    Red,
    Blue,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Red => write!(f, "Red"),
            Status::Blue => write!(f, "Blue"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Patch {
    pub id: u32,
    pub value: Status,
}

impl Hash for Patch {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Patch {}

impl PartialEq for Patch {
    fn eq(&self, other: &Patch) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} value {}", self.id, self.value)
    }
}

pub struct World {
    pub step: u64,
    pub field: SparseGrid2D<Patch>,
}

impl World {
    pub fn new() -> World {
        World {
            step: 0,
            field: SparseGrid2D::new(WIDTH, HEIGHT),
        }
    }

    pub fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    #[allow(dead_code)]
    pub fn as_state(&self) -> &dyn State {
        self
    }
}

impl State for World {
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }
    fn reset(&mut self) {
        self.step = 0;
        self.field = SparseGrid2D::new(WIDTH, HEIGHT);
    }

    fn init(&mut self, schedule: &mut rust_ab::engine::schedule::Schedule) {
        //println!("init system by state");
        self.step = 0;

        let mut rng = rand::thread_rng();

        for i in 0..NUM_AGENTS {
            let xx: i32 = rng.gen_range(0..WIDTH - 1);
            let yy: i32 = rng.gen_range(0..HEIGHT - 1);

            if i < ((NUM_AGENTS as f32) * PERC).ceil() as u32 {
                self.field.set_object_location(
                    Patch {
                        id: i,
                        value: Status::Red,
                    },
                    &Int2D { x: xx, y: yy },
                );
            } else {
                self.field.set_object_location(
                    Patch {
                        id: i,
                        value: Status::Blue,
                    },
                    &Int2D { x: xx, y: yy },
                );
            }
        }

        let agent = Updater { id: 0 };
        schedule.schedule_repeating(Box::new(agent), 0., 0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {}

    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1;
    }

    fn end_condition(&mut self, _schedule: &mut Schedule) -> bool {
        false
    }
}