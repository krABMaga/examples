use core::fmt;
use krabmaga::{
    engine::{agent::Agent, location::Int2D, state::State},
    rand,
    rand::Rng,
};
use std::hash::{Hash, Hasher};

use crate::model::state::{LifeState, WsgState};
use crate::{ENERGY_CONSUME, GAIN_ENERGY_WOLF, MOMENTUM_PROBABILITY, WOLF_REPR};

#[derive(Copy, Clone)]
pub struct Wolf {
    pub id: u32,
    pub animal_state: LifeState,
    pub loc: Int2D,
    pub last: Option<Int2D>,
    pub energy: f64,
    pub gain_energy: f64,
    pub prob_reproduction: f64,
}

impl Wolf {
    pub fn new(id: u32, loc: Int2D, energy: f64, gain_energy: f64, prob_reproduction: f64) -> Wolf {
        Wolf {
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

impl Agent for Wolf {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<WsgState>().unwrap();
        let x = self.loc.x;
        let y = self.loc.y;
        let mut rng = rand::thread_rng();

        // CHECK IF I AM DEAD
        if self.animal_state == LifeState::Dead {
            return;
        }

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

            let nx = rng.gen_range(xmin..=xmax);
            let ny = rng.gen_range(ymin..=ymax);
            self.loc = Int2D {
                x: x + nx,
                y: y + ny,
            };
            self.last = Some(Int2D { x, y });
        }

        state.wolves_grid.set_object_location(*self, &self.loc);

        //EAT
        if let Some(sheep) = state.sheep_grid.get_objects(&self.loc) {
            for mut sheep in sheep {
                if state.killed_sheep.get(&sheep).is_none()
                    && sheep.animal_state == LifeState::Alive
                {
                    sheep.animal_state = LifeState::Dead;
                    state.sheep_grid.remove_object_location(sheep, &sheep.loc);
                    self.energy += self.gain_energy;
                    state.killed_sheep.insert(sheep);
                    break;
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

                let new_wolf = Wolf::new(
                    state.next_id,
                    self.loc,
                    self.energy,
                    GAIN_ENERGY_WOLF,
                    WOLF_REPR,
                );

                state.next_id += 1;
                state.new_wolves.push(new_wolf);
            }
        }
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        self.animal_state == LifeState::Dead
    }
}

impl Eq for Wolf {}

impl PartialEq for Wolf {
    fn eq(&self, other: &Wolf) -> bool {
        self.id == other.id
    }
}

impl Hash for Wolf {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Wolf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
