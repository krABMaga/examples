use crate::model::forest::Forest;
use crate::model::forest::Status;
use crate::Tree;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Spread {
    pub id: u32,
}

impl Agent for Spread {
    fn step(&mut self, state: &mut dyn State) {
        let real_state = state.as_any().downcast_ref::<Forest>().unwrap();

        let updates = RefCell::new(Vec::<(Tree, Int2D)>::new());
        real_state.field.iter_objects(|loc, &(mut value)| {
            if loc.x <= real_state.step as i32 + 1 {
                let x = loc.x;
                let y = loc.y;
                if value.status == Status::Green {
                    // get the neighbors around me
                    let mut update = false;
                    for i in 0..3 {
                        for j in 0..3 {
                            if !(i == 1 && j == 1) {
                                let loc_n = Int2D {
                                    // location of neighbor
                                    x: x + i - 1,
                                    y: y + j - 1,
                                };
                                // not toroidal
                                if loc_n.x < 0
                                    || loc_n.y < 0
                                    || loc_n.x >= real_state.dim.0
                                    || loc_n.y >= real_state.dim.1
                                {
                                    continue;
                                };

                                // take the neighbor
                                let neighbor = match real_state.field.get_objects(&loc_n) {
                                    Some(t) => t[0],
                                    None => {
                                        continue;
                                    }
                                };
                                // if a neighbor is BURNING, set me on BURNING
                                if neighbor.status == Status::Burning {
                                    value.status = Status::Burning;
                                    //println!("I am {:?} passing on {:?} from {:?} step {}", value.id, value.status, neighbor.id, schedule.step);
                                    update = true;
                                    break; // avoid to be burned more than once
                                }
                            }
                        }
                        if update {
                            break;
                        }
                    }
                } else if value.status == Status::Burning {
                    // if I am BURNING, set me on BURNED
                    value.status = Status::Burned;
                    //println!("I am {:?} passing on {:?} step {}", value.id, value.status, schedule.step);
                }
            }
            updates.borrow_mut().push((value, *loc));
        });

        let updates = updates.borrow_mut();
        for obj in updates.iter() {
            real_state.field.set_object_location(obj.0, &obj.1);
        }
    }
}

impl Spread {
    #[allow(dead_code)]
    fn update(
        _loc: &Int2D,
        _value: &Tree,
        _state: &mut dyn State,
        _schedule: &mut Schedule,
        _schedule_id: u32,
    ) {
    }
}

impl Hash for Spread {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Spread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Spread {}

impl PartialEq for Spread {
    fn eq(&self, other: &Spread) -> bool {
        self.id == other.id
    }
}
