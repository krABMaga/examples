use crate::model::state::Environment;
use crate::model::state::Patch;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::{Schedule, ScheduleOptions};
use krabmaga::engine::state::State;
use krabmaga::Rng;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Eater {
    pub id: u32,
    pub position: Int2D,
    pub vision: u32,
    pub metabolism: u32,
    pub age: u32,
    pub max_age: u32,
    pub wealth: i32,
}

impl Agent for Eater {
    //Each step, the agent checks all free patches around it with the highest amount of sugar, based on its vision
    //If a free patch has been found, the agent moves inside it.
    //The agent then updates its state
    fn step(&mut self, state: &mut dyn State) {
        let mut rng = krabmaga::rand::thread_rng();
        let state = state.as_any_mut().downcast_mut::<Environment>().unwrap();

        if self.age == self.max_age || self.wealth <= 0 {
            let rand_x = rng.gen_range(0..state.dim.0);
            let rand_y = rng.gen_range(0..state.dim.1);
            let new_pos = Int2D {
                x: rand_x,
                y: rand_y,
            };
            let new_wealth = rng.gen_range(20..50);

            self.position = new_pos;
            self.wealth = new_wealth;
            self.age = 0;
            state.eaters.set_object_location(*self, &new_pos);
        }

        //println!("\n--------- Agent {} ---------", self.id);

        let mut range_x = self.position.x + self.vision as i32;
        let mut neg_range_x = self.position.x - self.vision as i32;
        let mut range_y = self.position.y + self.vision as i32;
        let mut neg_range_y = self.position.y - self.vision as i32;
        let mut near_patches: Vec<(Patch, Int2D)> = Vec::new();
        let mut max_sugar = 0;

        //Checks if the calculated range exceeds the field dimensions
        if range_x >= state.dim.0 {
            range_x = state.dim.0 - 1;
        }
        if neg_range_x < 0 {
            neg_range_x = 0;
        }
        if range_y >= state.dim.1 {
            range_y = state.dim.1 - 1
        }
        if neg_range_y < 0 {
            neg_range_y = 0;
        }

        //Finds the nearest patch with the highest amount of sugar
        for i in neg_range_x..range_x + 1 {
            for j in neg_range_y..range_y + 1 {
                let pos = Int2D { x: i, y: j };
                let obj = state.field.get_value(&pos);

                if let Some(patch) = obj {
                    let obj = state.eaters.get_objects_unbuffered(&pos);

                    //Takes the same patch where the agent is actually in
                    if obj.is_some()
                        && obj.unwrap()[0].id == self.id
                        && patch.sugar_amount >= max_sugar
                    {
                        max_sugar = patch.sugar_amount;
                        near_patches.retain(|&p| p.0.sugar_amount == patch.sugar_amount);
                        near_patches.push((patch, pos));
                    }
                    //Otherwise it check the others free patches
                    else if state.eaters.get_objects_unbuffered(&pos).is_none()
                        && patch.sugar_amount > 0
                        && patch.sugar_amount >= max_sugar
                    {
                        max_sugar = patch.sugar_amount;
                        near_patches.retain(|&p| p.0.sugar_amount == patch.sugar_amount);
                        near_patches.push((patch, pos));
                    }
                }
            }
        }

        //Chooses one random patch between all near patches
        //Updates the wealth of the agent based on its metabolism and on the sugar of the patch chosen
        //Updates the agent state and position into the field
        let len = near_patches.len();
        if len > 0 {
            let rand = rng.gen_range(0..len);
            let nearest_patch = near_patches[rand].0;
            let nearest_pos = near_patches[rand].1;
            // let p = state.field.get_value(&nearest_pos).unwrap();

            self.wealth += nearest_patch.sugar_amount as i32;
            self.wealth -= self.metabolism as i32;
            self.age += 1;
            //p.sugar_amount=0;
            //state.field.set_value_location(p, &nearest_pos);

            //print!("{} has moved from {};{} ", self.id,self.position.x, self.position.y);

            self.position = nearest_pos;
            state.eaters.set_object_location(*self, &self.position);

            //println!("to {};{}", self.position.x, self.position.y);
        }
    }

    //If the agent has 0 wealth or it exceedes its max age, it dies
    //When an agent dies it respawns in a random position
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }

    fn before_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }

    fn after_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }
}

impl Eater {
    #[allow(dead_code)]
    fn update(
        _loc: &Int2D,
        _value: &Patch,
        _state: &mut dyn State,
        _schedule: &mut Schedule,
        _schedule_id: u32,
    ) {
    }
}

impl Hash for Eater {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Eater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Eater {}

impl PartialEq for Eater {
    fn eq(&self, other: &Eater) -> bool {
        self.id == other.id
    }
}
