use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

use crate::model::state::*;
use crate::{
    HEIGHT, MOMENTUM_PROBABILITY, RANDOM_ACTION_PROBABILITY, REWARD, UPDATE_CUTDOWN, WIDTH,
};

// A struct representing an ant, with an id, a location, whether it's holding food or not and the
// current reward, used to increase the pheromone on the location of the ant if a site is reached.
#[derive(Copy, Clone)]
pub struct Ant {
    // An unique id.
    pub id: u32,
    // The location of the agent.
    pub loc: Int2D,
    // Last location of the agent, starts as None
    pub last: Option<Int2D>,
    // False means the agent will try to find food by following food pheromones if possible, or by
    // flooding the grid until it is found. True means the agent will try to return home by using the
    // previously deposited pheromones.
    pub has_food: bool,
    // Value used to increase the pheromones in the nest and in the food source.
    // This will let the agents spread pheromones in the surrounding areas from point of interests
    // so that other agents will know which path to take to do their job.
    pub reward: f32,
}

impl Ant {
    pub fn new(id: u32, loc: Int2D, has_food: bool, reward: f32) -> Ant {
        Ant {
            id,
            loc,
            last: None,
            has_food,
            reward,
        }
    }

    // Deposit a home pheromone if self is not holding food, else deposit a food pheromone,
    // so that other agents will take in account the pheromone value when choosing the next step's
    // direction.
    pub fn deposit_pheromone(&mut self, state: &ModelState) {
        let x = self.loc.x;
        let y = self.loc.y;

        // Fetch the value of the correct pheromone on our location, depending whether we're holding
        // food or not.
        let mut max = if self.has_food {
            state.to_food_grid.grid.get_value(&self.loc)
        } else {
            state.to_home_grid.grid.get_value(&self.loc)
        }
        .unwrap_or(0.);

        // Find the highest pheromone we care about in the surrounding 3x3 area to calculate the value
        // of the pheromone in our current area. Normally, the maximum pheromone in the 3x3 area is fetched
        // and it is decreased slightly, then it is assigned to our location.
        for dx in -1..2 {
            for dy in -1..2 {
                let _x = dx + x;
                let _y = dy + y;
                if _x < 0 || _y < 0 || _x >= WIDTH || _y >= HEIGHT {
                    // Do not try to take into account out of bounds grid cells
                    continue;
                }
                // Fetch the pheromone in the cell we're analyzing
                let pheromone = if self.has_food {
                    state.to_food_grid.grid.get_value(&Int2D { x: _x, y: _y })
                } else {
                    state.to_home_grid.grid.get_value(&Int2D { x: _x, y: _y })
                }
                .unwrap_or(0.);
                // Decrease the value a bit, with diagonal cells of our 3x3 grid considered farther
                let m = (pheromone * {
                    if dx * dy != 0 {
                        Ant::diagonal_cutdown()
                    } else {
                        UPDATE_CUTDOWN
                    }
                }) + self.reward;
                if m > max {
                    max = m;
                }
            }
        }
        // Set the new value of the pheromone we're considering
        if self.has_food {
            state.to_food_grid.grid.set_value_location(max, &self.loc);
        } else {
            state.to_home_grid.grid.set_value_location(max, &self.loc);
        }
        // We have used our reward, reset it
        self.reward = 0.;
    }

    // Step to the next cell by taking into account pheromones. If no pheromones of the right type
    // are found in a 3x3 grid centered on us, try to step in the same direction of the last frame
    // with a probability of MOMENTUM_PROBABILITY. Otherwise, step in a random direction with a
    // probability of RANDOM_ACTION_PROBABILITY.
    pub fn act(&mut self, state: &ModelState) {
        let mut rng = rand::thread_rng();
        let mut max = -1.; // An initial, impossible pheromone.

        let x = self.loc.x;
        let y = self.loc.y;

        let mut max_x = x;
        let mut max_y = y;
        let mut count = 2; // How many equal pheromones are there around us? Will be used to choose one randomly

        // Check a 3x3 grid centered on us to get a hint on where to step next through the pheromones around us
        for dx in -1..2 {
            for dy in -1..2 {
                let new_x = dx + x;
                let new_y = dy + y;
                let new_int2d = Int2D { x: new_x, y: new_y };
                // Skip the cell we're considering if we're trying to stay still, if we're trying
                // to exit the field or of we encounter an obstacle
                if (dx == 0 && dy == 0)
                    || new_x < 0
                    || new_y < 0
                    || new_x >= WIDTH
                    || new_y >= HEIGHT
                    || state.get_obstacle(&new_int2d).is_some()
                {
                    continue;
                }

                let m = if self.has_food {
                    state.to_home_grid.grid.get_value(&new_int2d)
                } else {
                    state.to_food_grid.grid.get_value(&new_int2d)
                }
                .unwrap_or(0.);
                if m > max {
                    // We found a new maximum, reset the count
                    count = 2;
                }
                // A new maximum is found, or the maximux hasn't changed. In the latter case, we
                // randomly choose whether to consider the new cell for the next step or not with an
                // equal chance.
                if m > max || (m == max && rng.gen_bool(1. / count as f64)) {
                    // Latter expression is to take a random step towards paths with a good pheromone
                    max = m;
                    max_x = new_x;
                    max_y = new_y;
                }
                count += 1;
            }
        }

        if max == 0. && self.last.is_some() {
            // No tips from pheromones, consider stepping in the same direction
            if let Some(last_loc) = self.last {
                if rng.gen_bool(MOMENTUM_PROBABILITY) {
                    let xm = x + (x - last_loc.x);
                    let ym = y + (y - last_loc.y);
                    // Don't go outside the field or in an obstacle
                    if (0..WIDTH).contains(&xm)
                        && (0..HEIGHT).contains(&ym)
                        && state.get_obstacle(&Int2D { x: xm, y: ym }).is_none()
                    {
                        max_x = xm;
                        max_y = ym;
                    }
                }
            }
        } else if rng.gen_bool(RANDOM_ACTION_PROBABILITY) {
            // All other ideas have failed, just choose a random direction
            let xd: i32 = rng.gen_range(-1..2);
            let yd: i32 = rng.gen_range(-1..2);
            let xm = x + xd;
            let ym = y + yd;
            // Don't go outside the field, in an obstacle and do not stay still
            if !(xd == 0 && yd == 0)
                && (0..WIDTH).contains(&xm)
                && (0..HEIGHT).contains(&ym)
                && state.get_obstacle(&Int2D { x: xm, y: ym }).is_none()
            {
                max_x = xm;
                max_y = ym;
            }
        }
        let loc = Int2D { x: max_x, y: max_y };
        self.loc = loc;
        state.ants_grid.set_object_location(*self, &loc);
        self.last = Some(Int2D { x, y });

        // Get rewarded if we've reached a site and update our food status
        if let Some(obs) = state.obstacles_grid.get_objects(&self.loc) {
            match obs.first().unwrap().value {
                ItemType::Home => {
                    if self.has_food {
                        {
                            let mut x = state.food_returned_home.write().unwrap();
                            *x = true;
                        }
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                ItemType::Food => {
                    if !self.has_food {
                        {
                            let mut x = state.food_source_found.write().unwrap();
                            *x = true;
                            //println!("Found food!");
                        }
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                ItemType::Obstacle => {}
            }
        }
    }

    fn diagonal_cutdown() -> f32 {
        UPDATE_CUTDOWN.powf((2_f32).sqrt())
    }
}

impl Agent for Ant {
    /// Each ant deposits a pheromone in its current location, then it steps in the next grid cell.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<ModelState>().unwrap();
        self.deposit_pheromone(state);
        self.act(state);
    }
}

impl Eq for Ant {}

impl PartialEq for Ant {
    fn eq(&self, other: &Ant) -> bool {
        self.id == other.id
    }
}

impl Hash for Ant {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.id, self.loc)
    }
}
