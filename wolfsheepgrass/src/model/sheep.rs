use crate::model::state::{LifeState, WsgState};
use crate::{ENERGY_CONSUME, FULL_GROWN, GAIN_ENERGY_SHEEP, MOMENTUM_PROBABILITY, SHEEP_REPR};

use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Sheep {
    pub id: u32,
    pub animal_state: LifeState,
    pub loc: Int2D,
    pub last: Option<Int2D>,
    pub energy: f64,
    pub gain_energy: f64,
    pub prob_reproduction: f64,
}

impl Sheep {
    pub fn new(
        id: u32,
        loc: Int2D,
        energy: f64,
        gain_energy: f64,
        prob_reproduction: f64,
    ) -> Sheep {
        Sheep {
            id,
            loc,
            last: None,
            energy,
            gain_energy,
            prob_reproduction,
            animal_state: LifeState::Alive,
        }
    }

    #[allow(dead_code)]
    pub fn as_agent(self) -> Box<dyn Agent> {
        Box::new(self)
    }
}

impl Agent for Sheep {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<WsgState>().unwrap();
        // CHECK IF I AM DEAD
        if self.animal_state == LifeState::Dead {
            return;
        }
        //MOVE
        let x = self.loc.x;
        let y = self.loc.y;
        let mut rng = rand::thread_rng();

        let mut moved = false;
        if self.last != None && rng.gen_bool(MOMENTUM_PROBABILITY) {
            if let Some(last_loc) = self.last {
                let xm = x + (x - last_loc.x);
                let ym = y + (y - last_loc.y);
                let new_loc = Int2D { x: xm, y: ym };
                // TRY TO MOVE WITH MOMENTUM_PROBABILITY
                if xm >= 0 && xm < state.dim.0 && ym >= 0 && ym < state.dim.1 {
                    self.loc = new_loc;
                    self.last = Some(Int2D { x, y });
                    moved = true;
                }
            }
        }

        if !moved {
            let xmin = if x > 0 { -1 } else { 0 };
            let xmax = if x < state.dim.0 - 1 { 1 } else { 0 };
            let ymin = if y > 0 { -1 } else { 0 };
            let ymax = if y < state.dim.1 - 1 { 1 } else { 0 };

            // let nx = if rng.gen_bool(0.5) { xmin } else { xmax };
            // let ny = if rng.gen_bool(0.5) { ymin } else { ymax };
            let nx = rng.gen_range(xmin..=xmax);
            let ny = rng.gen_range(ymin..=ymax);

            self.loc = Int2D {
                x: x + nx,
                y: y + ny,
            };
            self.last = Some(Int2D { x, y });
        }

        state.sheep_grid.set_object_location(*self, &self.loc);
        //EAT
        if state.grass_field.get_value_unbuffered(&self.loc).is_none() {
            if let Some(grass_val) = state.grass_field.get_value(&self.loc) {
                if grass_val >= FULL_GROWN {
                    state.grass_field.set_value_location(0, &self.loc);
                    self.energy += self.gain_energy;
                }
            }
        }

        //UPDATE ENERGY
        self.energy -= ENERGY_CONSUME;
        if self.energy <= 0.0 {
            self.animal_state = LifeState::Dead;
        } else {
            //REPRODUCE
            if rng.gen_bool(self.prob_reproduction) {
                self.energy /= 2.0;
                //let mut new_id = state.next_id;

                let new_sheep = Sheep::new(
                    state.next_id,
                    self.loc,
                    self.energy,
                    GAIN_ENERGY_SHEEP,
                    SHEEP_REPR,
                );

                state.next_id += 1;
                state.new_sheep.push(new_sheep);
            }
        }
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        self.animal_state == LifeState::Dead
    }
}

impl Eq for Sheep {}

impl PartialEq for Sheep {
    fn eq(&self, other: &Sheep) -> bool {
        self.id == other.id
    }
}

impl Hash for Sheep {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Sheep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
