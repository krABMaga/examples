use crate::model::animals::*;
use crate::model::state::State;
use crate::{HEIGHT, WIDTH};

use rust_ab::engine::location::Int2D;
use rust_ab::rand;
use rust_ab::rand::Rng;

pub const MOMENTUM_PROBABILITY: f64 = 0.8;

impl Animal {
    pub fn new_wolf(
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
            species: AnimalSpecies::Wolf,
        }
    }
}

impl Animal {
    pub fn wolf_eat(&mut self, state: &State) {
        if let Some(prey) = state.get_sheep_at_location(&self.loc) {
            if let Some(loc) = state.get_sheep_location(&prey){
                if loc.x != prey.loc.x || loc.y != prey.loc.y{
                    return
                }
                else {println!("same locs")}
            }else {println!("There is a dead sheep here!"); return;}

            match prey.animal_state {
                LifeState::Alive => {
                    let id_wolf = self.id;
                    let id_sheep = prey.id;


                    prey.die(state);
                    self.energy += self.gain_energy;

                    println!(
                        "Sheep{} eaten by Wolf{}  at step{}, sheep loc: {} {}, wolf loc: {} {}\n--------",
                        id_sheep, id_wolf, state.step, prey.loc.x, prey.loc.y, self.loc.x, self.loc.y
                    );

        
                    /*
                    println!("Sheep{} rimossa", prey.id);
                    if loc.is_some(){
                        println!("MA COME IS POSSIBLE");
                    }*/


                }


                LifeState::Dead => {}
            }
        }
    }

    pub fn wolf_act(&mut self, state: &State) {
        let x = self.loc.x;
        let y = self.loc.y;

        let mut rng = rand::thread_rng();

        let mut found_food: bool = false;
        let mut prey_loc = self.loc;

        for dx in -1..2 {
            for dy in -1..2 {
                //Calculate position to check
                let new_x = dx + x;
                let new_y = dy + y;
                let new_int2d = Int2D { x: new_x, y: new_y };

                if (dx == 0 && dy == 0)
                    || new_x < 0
                    || new_y < 0
                    || new_x >= WIDTH
                    || new_y >= HEIGHT
                {
                    continue;
                }

                let food = state.get_sheep_at_location(&new_int2d);
                

                if food.is_some() {
                    if let Some(checksum) = state.get_sheep_location(food.unwrap()){
                        //println!("loc1: {} {}  - loc2: {} {}", checksum.x, checksum.y, new_int2d.x, new_int2d.y);
                        if checksum.x != new_int2d.x || checksum.y != new_int2d.y{
                           // println!("Sheep trace, wrong location, loc1: {} {}  - loc2: {} {}", checksum.x, checksum.y, new_int2d.x, new_int2d.y);
                            continue;
                        }
                        else{
                            //println!("loc1: {} {}  - loc2: {} {}", checksum.x, checksum.y, new_int2d.x, new_int2d.y)
                        }
                    }
                    else {
                        //println!("It's a dead sheep, loc: {} {}", new_int2d.x, new_int2d.y);
                        continue;
                    }
                    
                    

                    match food.unwrap().animal_state {
                        LifeState::Alive => {
                            if !found_food {
                                found_food = true;
                                prey_loc = (*food.unwrap()).loc;
                            } else if rng.gen_bool(FOOD_CHOICE) {
                                prey_loc = (*food.unwrap()).loc;
                            }
                        }
                        LifeState::Dead => {
                            continue;
                        }
                    }
                }
            }
        }

        let mut loc = Int2D { x, y };
        if found_food {
            loc = prey_loc;
        } else if self.last != None && rng.gen_bool(MOMENTUM_PROBABILITY) {
            if let Some(last_pos) = self.last {
                let xm = x + (x - last_pos.x);
                let ym = y + (y - last_pos.y);
                // Don't go outside the field
                if xm >= 0 && xm < WIDTH && ym >= 0 && ym < HEIGHT {
                    loc = Int2D { x: xm, y: ym };
                }
            }
        } else {
            // All other ideas have failed, just choose a random direction
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
            state.set_wolf_location(self, &loc);
            self.last = Some(Int2D { x, y });
        }
    }
}
