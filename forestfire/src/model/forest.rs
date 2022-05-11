use crate::model::spread::Spread;
use core::fmt;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    Green,   // tree alive - the fire can spread here
    Burning, // burning tree - the fire is here
    Burned,  // burned tree - the fire stopped here and moved
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Green => write!(f, "Green"),
            Status::Burning => write!(f, "Burning"),
            Status::Burned => write!(f, "Burned"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tree {
    pub id: i32,
    pub status: Status,
}

impl Hash for Tree {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Tree {}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} status {}", self.id, self.status)
    }
}

pub struct Forest {
    pub step: u64,
    pub field: DenseGrid2D<Tree>,
    pub before_burned: i32,
    pub before_burning: i32,
    pub before_green: i32,
    pub burned: i32,
    pub burning: i32,
    pub green: i32,
    pub dim: (i32, i32),
    pub density: f64,
}

impl Forest {
    pub fn new(dim: (i32, i32), density: f64) -> Forest {
        Forest {
            step: 0,
            density,
            dim,
            field: DenseGrid2D::new(dim.0, dim.1),
            before_burned: 0,
            before_burning: 0,
            before_green: 0,
            burned: 0,
            burning: 0,
            green: 0,
        }
    }

    pub fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    #[allow(dead_code)]
    pub fn as_state(&self) -> &dyn State {
        self
    }
}

impl State for Forest {
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.field = DenseGrid2D::new(self.dim.0, self.dim.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();
        let mut ids = 0;
        // generate the trees to populate the forest
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                if rng.gen_bool(self.density) {
                    let mut status_tree = Status::Green;
                    if i == 0 {
                        // Set the trees at the left edge on fire
                        status_tree = Status::Burning;
                    }

                    self.field.set_object_location(
                        Tree {
                            id: ids,
                            status: status_tree,
                        },
                        &Int2D { x: i, y: j },
                    );
                    ids += 1;
                }
            }
        }
        let spreader = Spread { id: 0 };
        schedule.schedule_repeating(Box::new(spreader), 0., 0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1;
    }

    fn end_condition(&mut self, _schedule: &mut Schedule) -> bool {
        // for i in 0..self.field.width {
        //     for j in 0..self.field.height {
        //         let tree = match self.field.get_objects(&Int2D { x: i, y: j }) {
        //             Some(t) => t[0],
        //             None => {
        //                 continue;
        //             }
        //         };
        //         if tree.status == Status::Burned {
        //             self.burned += 1;
        //         } else if tree.status == Status::Green {
        //             self.green += 1;
        //         } else {
        //             self.burning += 1
        //         }
        //     }
        // }

        // if (self.before_burned == self.burned)
        //     && self.before_burning == self.burning
        //     && self.before_green == self.green
        // {
        //     println!("-- Simulation finished at step {:?} --\nTotal trees in the forest: Green {:?}, Burning {:?}, Burned {:?}\n",
        //         schedule.step, self.green, self.burning, self.burned
        //     );
        //     return true;
        // } else {
        //     println!("-- Simulation continues at step {:?} --\nTotal trees in the forest: Green {:?}, Burning {:?}, Burned {:?}\n",
        //         schedule.step, self.green, self.burning, self.burned
        //     );
        // }

        // self.before_burned = self.burned;
        // self.before_green = self.green;
        // self.before_burning = self.burning;

        // self.burned = 0;
        // self.green = 0;
        // self.burning = 0;

        false
    }
}
