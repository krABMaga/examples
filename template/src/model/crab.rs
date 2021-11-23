use crate::model::sea::Sea;
use core::fmt;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::fields::field_2d::toroidal_transform;
use rust_ab::engine::location::{Location2D, Real2D};
use rust_ab::engine::{schedule::Schedule, state::State};
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Crab {
    pub id: u32,
    pub pos: Real2D,
    pub last_d: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
}

impl Agent for Crab {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State, _schedule: &mut Schedule, _schedule_id: u32) {
        let state = state.as_any().downcast_ref::<Sea>().unwrap();
        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.5) {
            self.dir_x -= 1.0;
        }
        if rng.gen_bool(0.5) {
            self.dir_y -= 1.0;
        }

        let loc_x = toroidal_transform(self.pos.x + self.dir_x, state.field.width);
        let loc_y = toroidal_transform(self.pos.y + self.dir_y, state.field.heigth);
        self.pos = Real2D { x: loc_x, y: loc_y };

        state
            .field
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Crab {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Location2D<Real2D> for Crab {
    fn get_location(self) -> Real2D {
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for Crab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Crab {}

impl PartialEq for Crab {
    fn eq(&self, other: &Crab) -> bool {
        self.id == other.id
    }
}
