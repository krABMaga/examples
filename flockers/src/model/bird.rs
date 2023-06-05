#![allow(warnings)]
use crate::lazy_static;
use crate::mpi::datatype::UncommittedDatatypeRef;
use crate::mpi::environment::Universe;
use crate::mpi::ffi::MPI_Finalize;
use crate::mpi::point_to_point::Destination;
use crate::mpi::point_to_point::Source;
use crate::mpi::topology::Communicator;
use crate::mpi::topology::SystemCommunicator;
use crate::mpi::traits::*;
use crate::mpi::Address;
use crate::mpi::Threading;
use crate::p2p::ReceiveFuture;
use crate::UserDatatype;
use core::fmt;
use core::mem::size_of;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::kdtree_mpi::{toroidal_distance, toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::universe;
//use krabmaga::log;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

use crate::model::state::Flocker;
use crate::{AVOIDANCE, COHESION, CONSISTENCY, JUMP, MOMENTUM, RANDOMNESS};

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
}

unsafe impl Equivalence for Bird {
    type Out = UserDatatype;
    fn equivalent_datatype() -> Self::Out {
        UserDatatype::structured(
            &[1, 1, 1],
            &[
                (size_of::<Real2D>() * 2) as crate::mpi::Address,
                (size_of::<Real2D>()) as crate::mpi::Address,
                size_of::<u32>() as crate::mpi::Address,
            ],
            &[
                Real2D::equivalent_datatype(),
                Real2D::equivalent_datatype(),
                u32::equivalent_datatype(),
            ],
        )
    }
}

impl Bird {
    pub fn new(id: u32, loc: Real2D, last_d: Real2D) -> Self {
        Bird { id, loc, last_d }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State) {
        println!(" agente {}", self);
        let state = state.as_any_mut().downcast_mut::<Flocker>().unwrap();

        let world = universe.world();

        let status = world
            .any_process()
            .immediate_probe_with_tag(world.rank() + 90);
        match status {
            Some(e) => {
                //println!("Sono il processo {} e ho ricevuto {:?}", world.rank(), e);
                let (bird, _) = world.process_at_rank(e.source_rank()).receive::<Bird>();
                state.field1.insert(bird, bird.loc);
                //println!("{}",msg);
            }
            None => {}
        }

        let vec = state
            .field1
            .get_distributed_neighbors_within_relax_distance(self.loc, 10.0, self.clone());

        let width = state.dim.0;
        let height = state.dim.1;

        let mut avoidance = Real2D { x: 0.0, y: 0.0 };

        let mut cohesion = Real2D { x: 0.0, y: 0.0 };
        let mut randomness = Real2D { x: 0.0, y: 0.0 };
        let mut consistency = Real2D { x: 0.0, y: 0.0 };

        if !vec.is_empty() {
            //avoidance
            let mut x_avoid = 0.0;
            let mut y_avoid = 0.0;
            let mut x_cohe = 0.0;
            let mut y_cohe = 0.0;
            let mut x_cons = 0.0;
            let mut y_cons = 0.0;
            let mut count = 0;

            for elem in vec.iter() {
                let elem = *elem;
                if self.id != elem.id {
                    let dx = toroidal_distance(self.loc.x, elem.loc.x, width);
                    let dy = toroidal_distance(self.loc.y, elem.loc.y, height);
                    count += 1;

                    //avoidance calculation
                    let square = dx * dx + dy * dy;
                    x_avoid += dx / (square * square + 1.0);
                    y_avoid += dy / (square * square + 1.0);

                    //cohesion calculation
                    x_cohe += dx;
                    y_cohe += dy;

                    //consistency calculation
                    x_cons += elem.last_d.x;
                    y_cons += elem.last_d.y;
                }
            }

            if count > 0 {
                x_avoid /= count as f32;
                y_avoid /= count as f32;
                x_cohe /= count as f32;
                y_cohe /= count as f32;
                x_cons /= count as f32;
                y_cons /= count as f32;

                consistency = Real2D {
                    x: x_cons / count as f32,
                    y: y_cons / count as f32,
                };
            } else {
                consistency = Real2D {
                    x: x_cons,
                    y: y_cons,
                };
            }

            avoidance = Real2D {
                x: 400.0 * x_avoid,
                y: 400.0 * y_avoid,
            };

            cohesion = Real2D {
                x: -x_cohe / 10.0,
                y: -y_cohe / 10.0,
            };

            //randomness
            let mut rng = rand::thread_rng();
            let r1: f32 = rng.gen();
            let x_rand = r1 * 2.0 - 1.0;
            let r2: f32 = rng.gen();
            let y_rand = r2 * 2.0 - 1.0;

            let square = (x_rand * x_rand + y_rand * y_rand).sqrt();
            randomness = Real2D {
                x: 0.05 * x_rand / square,
                y: 0.05 * y_rand / square,
            };
        }

        let mom = self.last_d;

        let mut dx = COHESION * cohesion.x
            + AVOIDANCE * avoidance.x
            + CONSISTENCY * consistency.x
            + RANDOMNESS * randomness.x
            + MOMENTUM * mom.x;
        let mut dy = COHESION * cohesion.y
            + AVOIDANCE * avoidance.y
            + CONSISTENCY * consistency.y
            + RANDOMNESS * randomness.y
            + MOMENTUM * mom.y;

        let dis = (dx * dx + dy * dy).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
        }

        self.last_d = Real2D { x: dx, y: dy };

        let loc_x = toroidal_transform(self.loc.x + dx, width);
        let loc_y = toroidal_transform(self.loc.y + dy, height);

        self.loc = Real2D { x: loc_x, y: loc_y };
        drop(vec);
        let id = state.field1.get_block_by_location(self.loc.x, self.loc.y);
        if id as i32 == world.rank() {
            state.field1.insert(*self, self.loc);
        } else {
            //println!("Sono {} ed invio perché {};{} ha id {}", world.rank(), self.loc.x, self.loc.y, id);
            world
                .process_at_rank(id as i32)
                .send_with_tag(self, (id as i32) + 90);
        }
    }
}

/* if world.rank()!=0{
    let future: ReceiveFuture<Bird> = world.process_at_rank(0).immediate_receive();
    match future.r#try() {
        Ok((bird, _)) => {
            println!("Arrivato");
        }
        Err(e) =>{
            println!("Errore");
        }
    }
}

let id = state.field1.get_block_by_location(loc_x, loc_y);
if id as i32== world.rank(){
    state
    .field1
    .insert(*self, loc_x, loc_y);
}
else{
    world.process_at_rank(id as i32).send(self);
}

*/

impl Hash for Bird {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Bird {}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for Bird {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.id, self.loc)
    }
}
