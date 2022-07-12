use crate::model::world::World;
use crate::Patch;
use crate::SIMILAR_WANTED;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::{Schedule, ScheduleOptions};
use krabmaga::engine::state::State;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Updater {
    pub id: u32,
}

impl Agent for Updater {
    fn step(&mut self, state: &mut dyn State) {
        let real_state = state.as_any().downcast_ref::<World>().unwrap();
        let updates = RefCell::new(Vec::<(Patch, Int2D)>::new());

        real_state.field.iter_objects(|loc, value| {
            let x = loc.x;
            let y = loc.y;
            //let mut neighbors = 0.0;
            let mut similar = 0;

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

                        //neighbors += 1.0;

                        if value.value == neighbor.value {
                            similar += 1;
                        }
                    }
                }
            }
            let mut updates = updates.borrow_mut();

            if similar < SIMILAR_WANTED {
                let nloc = real_state.field.get_random_empty_bag();
                match nloc {
                    Some(rloc) => {
                        updates.push((*value, rloc));
                    }
                    None => {
                        updates.push((*value, *loc));
                    }
                }
            } else {
                // agent ok nothing to do
                updates.push((*value, *loc));
            }
        });

        let mut updates = updates.borrow_mut();
        for obj in updates.iter() {
            real_state.field.set_object_location(obj.0, &obj.1);
        }

        updates.clear();
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
