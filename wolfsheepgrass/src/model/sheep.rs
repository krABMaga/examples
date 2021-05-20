use crate::model::animals::*;

use crate::model::state::State;
use crate::{HEIGHT, WIDTH};

use rust_ab::engine::location::Int2D;
use rust_ab::rand;
use rust_ab::rand::Rng;

use crate::model::grass::*;

pub const MOMENTUM_PROBABILITY: f64 = 0.6;

impl Animal {
    pub fn new_sheep(
        id: u128,
        loc: Int2D,
        energy: f64,
        gain_energy: f64,
        prob_reproduction: f64,
    ) -> Animal {
        Animal {
            id,
            loc,
            last: None,
            energy,
            gain_energy,
            prob_reproduction,
            animal_state: LifeState::Alive,
            species: AnimalSpecies::Sheep,
        }
    }
}

impl Animal {
    pub fn sheep_act(&mut self, state: &State) {
        let x = self.loc.x;
        let y = self.loc.y;

        let mut rng = rand::thread_rng();
        let mut loc = Int2D { x, y };

        if self.last != None && rng.gen_bool(MOMENTUM_PROBABILITY) {
            if let Some(last_pos) = self.last {
                let xm = x + (x - last_pos.x);
                let ym = y + (y - last_pos.y);
                // Don't go outside the field
                if xm >= 0 && xm < WIDTH && ym >= 0 && ym < HEIGHT {
                    loc = Int2D { x: xm, y: ym };
                }
            }
        } else {
            let xd: i64 = rng.gen_range(-1..2);
            let yd: i64 = rng.gen_range(-1..2);
            let xm = x + xd;
            let ym = y + yd;
            // Don't go outside the field and do not stay still
            if !(xd == 0 && yd == 0) && xm >= 0 && xm < WIDTH && ym >= 0 && ym < HEIGHT {
                loc = Int2D { x: xm, y: ym };
            }
        }

        if loc.x != x || loc.y != y {
            self.loc = loc;
            state.set_sheep_location(&self, &loc);
            self.last = Some(Int2D { x, y });
        }
    }

    pub fn sheep_eat(&mut self, state: &State) {
        let loc = self.loc;

        if let Some(grass_val) = state.get_grass_at_location(&self.loc) {
            let mut grass_state = *grass_val;
            if grass_state != FULL_GROWN {
                return;
            }
            grass_state = 0;
            state.set_grass_at_location(&loc, grass_state);
            self.energy += self.gain_energy;
            //let g = state.get_grass_at_location(&loc).unwrap();
        }
    }
}
