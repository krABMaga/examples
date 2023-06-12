#![allow(warnings)]
use crate::lazy_static;

extern crate mpi;
use mpi::datatype::UncommittedDatatypeRef;
use mpi::datatype::UncommittedUserDatatype;
use mpi::environment::Universe;
use mpi::ffi::MPI_Finalize;
use mpi::point_to_point::Destination;
use mpi::point_to_point::Source;
use mpi::topology::Communicator;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use mpi::Address;
use mpi::internal::memoffset::{offset_of, span_of};
use mpi::Threading;
use crate::p2p::ReceiveFuture;
use crate::UserDatatype;
use core::fmt;
use core::mem::size_of;
use std::borrow::Borrow;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::kdtree_mpi::{toroidal_distance, toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::universe;
//use krabmaga::log;
use krabmaga::rand::Rng;
use mpi::traits::Equivalence;
use std::hash::{Hash, Hasher};

use crate::model::state::Flocker;
use crate::{AVOIDANCE, COHESION, CONSISTENCY, JUMP, MOMENTUM, RANDOMNESS};

#[derive(Clone, Copy, Equivalence)]
pub struct Bird {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
}

// unsafe impl Equivalence for Bird {
//     type Out = UserDatatype;
//     fn equivalent_datatype() -> Self::Out {
//         UserDatatype::structured(
//             &[1, 1, 1],
//             &[
//                 (size_of::<Real2D>()*2) as crate::mpi::Address,
//                 (size_of::<Real2D>()) as crate::mpi::Address,
//                 size_of::<u32>() as crate::mpi::Address,
//             ],
//             &[
//                 Real2D::equivalent_datatype(),
//                 Real2D::equivalent_datatype(),
//                 u32::equivalent_datatype(),
//             ],
//         )
//     }
// }
// unsafe impl Equivalence for Bird {
//     type Out = UserDatatype;
//     fn equivalent_datatype() -> Self::Out {
//         UserDatatype::structured(
//             &[1, 1, 1],
//             &[
//                 size_of::<u32>() as crate::mpi::Address,
//                 offset_of!(Bird, loc) as Address,
//                 offset_of!(Bird, last_d) as Address,
//             ],
//             &[
//                 u32::equivalent_datatype(),
//                 UserDatatype::structured(
//                     &[1, 1],
//                     &[
//                         size_of::<f32>() as crate::mpi::Address,
//                         size_of::<f32>() as crate::mpi::Address,
//                     ],
//                     &[f32::equivalent_datatype(), f32::equivalent_datatype()],
//                 ).as_ref(),
//                 UserDatatype::structured(
//                     &[1, 1],
//                     &[
//                         size_of::<f32>() as crate::mpi::Address,
//                         size_of::<f32>() as crate::mpi::Address,
//                     ],
//                     &[f32::equivalent_datatype(), f32::equivalent_datatype()],
//                 ).as_ref(),
//             ],
//         )
//     }
// }

impl Bird {
    pub fn new(id: u32, loc: Real2D, last_d: Real2D) -> Self {
        Bird { id, loc, last_d }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State) {
        //println!("inizio step  ");
        let state = state.as_any_mut().downcast_mut::<Flocker>().unwrap();

        let world = universe.world();

        // println!(" Sono  {} e sto gestendo agente {}",world.rank(), self);


        /* loop {
            let status = world
            .any_process()
            .immediate_probe_with_tag(world.rank() + 90);
            match status {
                Some(e) => {
                    let (bird, _) = world.process_at_rank(e.source_rank()).receive::<Bird>();
                    //println!("Sono il processo {} e ho ricevuto {} tag {:?}", world.rank(), bird, e);
                    state.field1.insert_read(bird, bird.loc);
                    //state.field1.insert(bird, bird.loc);
                    state.field1.agents_to_schedule.insert(bird);
                    //println!("{}",msg);
                }
                None => {break;}
            }
        } */
        
        let mut vec = state
            .field1
            .get_neighbors_within_distance(self.loc, 10.0);


        //println!("Sono il processo {} agente {} e ho {} vicini", world.rank(), self, vec.len());
        /* for neighbor in vec.iter(){
            println!("{}", neighbor.loc);
        } */
        // // print the neighbors
        // dedup the neighbors
        // vec.dedup_by(|a, b| a.id == b.id);
        // println!("Sono il processo {} agente {} e ho {} vicini", world.rank(), self, vec.len());
        // for elem in vec.iter() {
        //     print!("{}-", elem);
        // }
        // println!();
        let width = state.dim.0;
        let height = state.dim.1;

        /*let mut avoidance = Real2D { x: 0.0, y: 0.0 };

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
        let loc_y = toroidal_transform(self.loc.y + dy, height); */

        //self.loc = Real2D { x: loc_x, y: loc_y };

        let loc_x = toroidal_transform(self.loc.x + 1., width);
        let loc_y = toroidal_transform(self.loc.y, height); 
        self.loc = Real2D { x: loc_x, y: loc_y };
        drop(vec);
        let id = state.field1.get_block_by_location(self.loc.x, self.loc.y);
        //println!("Per bird {} ho trovato id {}", self, id);
        if id as i32 == world.rank() {
            //println!("Sono {} agente {} aggiorna loc", world.rank(), self);
            state.field1.insert(*self, self.loc);
        } else {
            //println!("Sono {} ed invio perché {} ha id {}", world.rank(), self, id);
            /* world
                .process_at_rank(id as i32)
                .send_with_tag(self, (id as i32) + 90); */
            //state.field1.killed_agent.insert(self.clone());
            state.field1.agents_to_send[id as usize].push(self.clone());
        }
    }
}


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
