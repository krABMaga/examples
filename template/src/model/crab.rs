use crate::model::sea::Sea;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::{toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Crab {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
}

impl Agent for Crab {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Sea>().unwrap();
        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.5) {
            self.dir_x -= 1.0;
        }
        if rng.gen_bool(0.5) {
            self.dir_y -= 1.0;
        }

        let loc_x = toroidal_transform(self.loc.x + self.dir_x, state.field.width);
        let loc_y = toroidal_transform(self.loc.y + self.dir_y, state.field.height);
        self.loc = Real2D { x: loc_x, y: loc_y };

        state
            .field
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
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
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
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
