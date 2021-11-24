use crate::model::world::World;
use crate::Patch;
use crate::PERCENT_SIMILAR_WANTED;
use core::fmt;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::location::Int2D;
use rust_ab::engine::schedule::{Schedule, ScheduleOptions};
use rust_ab::engine::state::State;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Updater {
    pub id: u32,
}

impl Agent for Updater {
    fn step(&mut self, state: &mut dyn State, _schedule: &mut Schedule, _schedule_id: u32) {
        let real_state = state.as_any().downcast_ref::<World>().unwrap();

        // let mut empty_bags = RefCell::new(Vec::new());

        let updates = RefCell::new(Vec::<(Patch, Int2D)>::new());

        real_state.field.iter_objects(|loc, value| {
            let x = loc.x;
            let y = loc.y;
            let mut neighbors = 0.0;
            let mut similar = 0.0;

            for i in 0..3 {
                for j in 0..3 {
                    if !(i == 1 && j == 1) {
                        let loc_n = Int2D {
                            x: x + j - 1,
                            y: y + i - 1,
                        };
                        if loc_n.x < 0
                            || loc_n.y < 0
                            || loc_n.x >= real_state.dim.0
                            || loc_n.y >= real_state.dim.0
                        {
                            continue;
                        };

                        let neighbor = match real_state.field.get_objects(&loc_n) {
                            Some(t) => t[0],
                            None => continue,
                        };

                        neighbors += 1.0;

                        if value.value == neighbor.value {
                            similar += 1.0;
                        }
                    }
                }
            }
            let mut updates = updates.borrow_mut();

            if neighbors == 0.0 || (similar / neighbors) < PERCENT_SIMILAR_WANTED {
                // agent not ok move to random place
                // let mut bags = empty_bags.borrow_mut();
                // if bags.len() == 0 {
                //     *bags = real_state.field.get_empty_bags();
                // }
                // let nindex = rng.gen_range(0..bags.len());
                // let nloc = bags[nindex];
                // bags.remove(nindex);

                let nloc = real_state.field.get_random_empty_bag();
                match nloc {
                    Some(rloc) => {
                        updates.push((*value, rloc));
                        // println!("Use a random empty bag");
                    }
                    None => {
                        updates.push((*value, *loc));
                        // println!("Random empty bag not found");
                    }
                }
                // real_state.field.set_object_location(*value, &nloc);

                // println!("{:?} set 2", loc);
                // println!("change loc {:?} {:?}",loc, nloc);
            } else {
                // agent ok nothing to do
                // real_state.field.set_object_location(*value, &loc);
                updates.push((*value, *loc));

                // println!("{:?} set 3", loc);
            }
        });

        let mut updates = updates.borrow_mut();
        for obj in updates.iter() {
            real_state.field.set_object_location(obj.0, &obj.1);
        }

        updates.clear();
    }

    fn get_id(&self) -> u32 {
        self.id
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

impl Updater {
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

impl Hash for Updater {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Updater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Updater {}

impl PartialEq for Updater {
    fn eq(&self, other: &Updater) -> bool {
        self.id == other.id
    }
}
