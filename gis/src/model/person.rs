use crate::model::map::Map;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::{toroidal_transform, Location2D};
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Person {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
}

impl Agent for Person {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Map>().unwrap();
        let gis_field = &state.gis_field;
        let field = &state.field;
        let last_d = self.last_d;
        /* let mut rng = rand::thread_rng();
        let generated = rng.gen_range(0..3);
        let next_x = last_d.x as i32 + 1;
        let next_y = last_d.y as i32 + 1;
        let prev_x = last_d.x as i32 - 1;
        let prev_y = last_d.y as i32 - 1;

        match generated {
            0 => {
                //in this case we do a simple check for the next cell in x-axis
                if gis_field
                    .get_value(&Int2D {
                        x: next_x,
                        y: last_d.y,
                    })
                    .eq(&Some(0))
                {
                    field.set_object_location(
                        curr_agent,
                        Real2D {
                            x: next_x as f32,
                            y: last_d.y as f32,
                        },
                    );
                }
            }
            1 => {
                //in this case we do a simple check for the next cell in y-axis
                if gis_field
                    .get_value(&Int2D {
                        x: next_y,
                        y: last_d.x,
                    })
                    .eq(&Some(0))
                {
                    field.set_object_location(
                        curr_agent,
                        Real2D {
                            x: next_y as f32,
                            y: last_d.x as f32,
                        },
                    );
                }
            }
            2 => {
                //in this case we do a simple check for the prev cell in x-axis
                if gis_field
                    .get_value(&Int2D {
                        x: prev_x,
                        y: last_d.y,
                    })
                    .eq(&Some(0))
                {
                    field.set_object_location(
                        curr_agent,
                        Real2D {
                            x: prev_x as f32,
                            y: last_d.y as f32,
                        },
                    );
                }
            }
            3 => {
                //in this case we do a simple check for the prev cell in y-axis
                if gis_field
                    .get_value(&Int2D {
                        x: prev_y,
                        y: last_d.x,
                    })
                    .eq(&Some(0))
                {
                    field.set_object_location(
                        curr_agent,
                        Real2D {
                            x: prev_y as f32,
                            y: last_d.x as f32,
                        },
                    );
                }
            }
            _ => todo!(),
        } */
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Person {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Person {}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for Person {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}
