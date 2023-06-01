#![allow(warnings)]
use crate::model::bird::Bird;
use crate::DISCRETIZATION;
//use crate::{DISCRETIZATION, TOROIDAL};
use crate::lazy_static;
use crate::mpi::datatype::UserDatatype;
use crate::mpi::environment::Universe;
use crate::mpi::ffi::MPI_Finalize;
use crate::mpi::point_to_point::Destination;
use crate::mpi::point_to_point::Source;
use crate::mpi::topology::Communicator;
use crate::mpi::topology::SystemCommunicator;
use crate::mpi::traits::*;
use crate::mpi::Address;
use crate::mpi::Threading;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::kdtree_mpi::Kdtree;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use krabmaga::universe;
use std::any::Any;

pub struct Flocker {
    pub step: u64,
    pub field1: Kdtree<Bird>,
    pub initial_flockers: u32,
    pub dim: (f32, f32),
}

impl Flocker {
    #[allow(dead_code)]
    pub fn new(dim: (f32, f32), initial_flockers: u32) -> Self {
        Flocker {
            step: 0,
            field1: Kdtree::create_tree(0, 0.0, 0.0, dim.0, dim.1, DISCRETIZATION, 25.),
            initial_flockers,
            dim,
        }
    }
}

impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Kdtree::create_tree(0, 0.0, 0.0, self.dim.0, self.dim.1, DISCRETIZATION ,25.)
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let world = universe.world();
        let mut rng = rand::thread_rng();

        // Should be moved in the init method on the model exploration changes
        if world.rank() == 0 {
            let mut vec: Vec<Vec<Bird>> = Vec::new();
            for _ in 1..world.size() {
                vec.push(vec![])
            }
            for bird_id in 0..self.initial_flockers {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let last_d = Real2D { x: 0., y: 0. };
                let loc = Real2D {
                    x: self.dim.0 * r1,
                    y: self.dim.1 * r2,
                };
                let mut bird = Bird::new(bird_id, loc, last_d);
                let mut id = self.field1.get_block_by_location(loc.x, loc.y);
                bird = Bird::new(bird_id, loc, last_d);
                //println!("Trovato blocco: {} per l'agente {};{}", id, loc.x, loc.y);
                if id > 0 {
                    vec[(id - 1) as usize].push(bird);
                } else {
                    self.field1.insert(bird, loc);
                    schedule.schedule_repeating(Box::new(bird), 0., 0);
                }
                if bird_id == self.initial_flockers - 1 {
                    for i in 1..world.size() {
                        world.process_at_rank(i).send(&vec[(i - 1) as usize]);
                    }
                    //println!("Sono il proc {} e ho un vec di {} elementi", world.rank(), 400 - vec[0].len()- vec[1].len()- vec[2].len());
                }
            }
        } else {
            let (vec, _) = world.process_at_rank(0).receive_vec::<Bird>();
            //println!("Sono il proc {} e ho ricevuto un vec di {} elementi", world.rank(), vec.len());
            for bird in vec.iter() {
                self.field1.insert(*bird, bird.loc);
                schedule.schedule_repeating(Box::new(*bird), 0., 0);
            }
        }
    }

    fn update(&mut self, _step: u64) {
        self.field1.lazy_update();
    }

    fn as_any(&self) -> &dyn Any {
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
}
