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
use mpi::request::WaitGuard;
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
                    let (counting, _) = schedule.distributed_schedule_repeating(Box::new(bird), 0., 0);
                    self.field1.scheduled_agent.insert(bird.id, counting);
                }
                if bird_id == self.initial_flockers - 1 {
                    for i in 1..world.size() {
                        world.process_at_rank(i).send(&vec[(i - 1) as usize]);
                    }
                    //println!("Sono il proc {} e ho un vec di {} elementi", world.rank(), 400 - vec[0].len()- vec[1].len()- vec[2].len());
                }
            } 

            /* for bird_id in 0..self.initial_flockers {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let last_d = Real2D { x: 0., y: 0. };
                let mut loc :Real2D = Real2D { x: 0., y: 0. };
                if bird_id == 0{
                    loc = Real2D {
                        x: 3.,
                        y: 2.,
                    };
                }
                if bird_id == 1{
                    loc = Real2D {
                        x: 4.,
                        y: 4.,
                    };
                }
                if bird_id == 2{
                    loc = Real2D {
                        x: 1.,
                        y: 8.,
                    };
                }
                if bird_id == 3{
                    loc = Real2D {
                        x: 6.,
                        y: 7.,
                    };
                }
                
                let mut bird = Bird::new(bird_id, loc, last_d);
                let mut id = self.field1.get_block_by_location(loc.x, loc.y);
                bird = Bird::new(bird_id, loc, last_d);
                //println!("Trovato blocco: {} per l'agente {};{}", id, loc.x, loc.y);
                if id > 0 {
                    vec[(id - 1) as usize].push(bird);
                } else {
                    self.field1.insert(bird, loc);
                    let (counting, _) = schedule.distributed_schedule_repeating(Box::new(bird), 0., 0);
                    self.field1.scheduled_agent.insert(bird.id, counting);
                }
                if bird_id == self.initial_flockers - 1 {
                    for i in 1..world.size() {
                        world.process_at_rank(i).send(&vec[(i - 1) as usize]);
                    }
                    //println!("Sono il proc {} e ho un vec di {} elementi", world.rank(), 400 - vec[0].len()- vec[1].len()- vec[2].len());
                }
            } */
        } else {
            let (vec, _) = world.process_at_rank(0).receive_vec::<Bird>();
            //println!("Sono il proc {} e ho ricevuto un vec di {} elementi", world.rank(), vec.len());
            for bird in vec.iter() {
                self.field1.insert(*bird, bird.loc);
                let (counting, _) = schedule.distributed_schedule_repeating(Box::new(*bird), 0., 0);
                self.field1.scheduled_agent.insert(bird.id, counting);
            }
        }
    }

    fn update(&mut self, _step: u64) {
        //println!("inizio update");
        self.field1.lazy_update();
         let mut vec = Vec::new();
        for loc in &self.field1.locs{
            for l in loc.borrow().iter(){
                for bird in l{
                    vec.push(bird.clone());
                }
            }
        }

        println!("Sono {} e nello step {} ho {} agenti", universe.world().rank(), _step, vec.len());
       
    }

    fn before_step(&mut self,schedule: &mut Schedule) {
        //println!("Inizio before_step");


        let mut vec = Vec::new();
        for loc in &self.field1.locs{
            for l in loc.borrow().iter(){
                for bird in l{
                    vec.push(bird.clone());
                }
            }
        }
        

        //println!("Sono {} e inizio lo step {} con {} agenti", universe.world().rank(), schedule.step, vec.len());
        //println!("Finisco before_step");
    } 
    
    fn after_step(&mut self,schedule: &mut Schedule) {
        //println!("Inizio after_step");
        let world=universe.world();
        let mut i = 0;

        for agent in &self.field1.agents_to_send{
            for bird in agent{
                //println!("Proc {} Agent in agents_to_send {}", world.rank(), bird);
            }
            
        }


        let dummy = Bird { id: 0, loc: Real2D { x: 0., y: 0. }, last_d: Real2D { x: 0., y: 0. } };

        let mut received_messages:Vec<usize> = vec![0; world.size() as usize];
        let mut send_vec: Vec<usize> = vec![0; world.size() as usize];
        let mut send_agent_vec: Vec<Vec<Bird>> = vec![vec![];world.size() as usize];


        for neighbor in &self.field1.neighbor_trees{
            send_vec[*neighbor as usize] += self.field1.agents_to_send[*neighbor as usize].len();
            send_agent_vec[*neighbor as usize].extend(self.field1.agents_to_send[*neighbor as usize].iter())
        }

        for neighbor in &self.field1.neighbor_trees{
            mpi::request::scope(|scope| {
                let ln = &send_vec[*neighbor as usize];
                let rreq = WaitGuard::from(world.process_at_rank(*neighbor).immediate_receive_into_with_tag(scope, &mut received_messages[*neighbor as usize], *neighbor));
                //println!("Process {} is ready to receive the message from {}", world.rank(), neighbor);
                let sreq = WaitGuard::from(world.process_at_rank(*neighbor).immediate_ready_send_with_tag(scope, ln , world.rank()));
                //println!("Process {} has sent value {} to {}", world.rank(), ln, neighbor);
            });
        }

        let mut vec:Vec<Vec<Bird>> = vec![vec![]; world.size() as usize];
        // println!("Sono {} e ho ricevuto {:?} world.size {:?}", world.rank(), received_messages, world.size());
        if received_messages.len()>0{
            for i in &self.field1.neighbor_trees{
                if received_messages[*i as usize] != 0{
                    //println!("Sono {} e mi aspetto di ricevere {} agenti da {}", world.rank(), received_messages[*i as usize], i);
                    vec[*i as usize].append(&mut vec![dummy; received_messages[*i as usize]]);
                }
                else {
                    //println!("Sono nell'else");
                    vec[*i as usize].append((&mut vec![]));
                }

            }
        }

        
        
        mpi::request::multiple_scope(world.size() as usize, |scope, coll| {
            let rank = (world.rank() + 1) % 2;
            for (id, buffer) in vec.iter_mut().enumerate(){
                if received_messages[id as usize] != 0{
                    let rreq = world.process_at_rank(id as i32).immediate_receive_into_with_tag(scope, &mut buffer[..], world.rank()+50);
                    coll.add(rreq);
                    //println!("Process {} is ready to receive {} agents from {}", world.rank(), received_messages[id as usize], id);
                }
            }

            for id in self.field1.neighbor_trees.iter(){
                if send_agent_vec[*id as usize].len() != 0{
                    let mut sreq = world.process_at_rank(*id).immediate_send_with_tag(scope, &send_agent_vec[*id as usize][..], *id+50);
                    coll.add(sreq);
                }
                
                //println!("Process {} has sent the vector of size {} to {}", world.rank(), &send_agent_vec[*id as usize].len(), id); 
            }
            
            let mut out = vec![];
            coll.wait_all(&mut out);
        }); 

        for vec in &self.field1.agents_to_send{
            if vec.len()!=0{
                for agent in vec{
                    //println!("Sono il proc {} e Agente {} è uscito dal campo perché ha id {}",world.rank(),agent, i );
                    let a = self.field1.scheduled_agent.get(&agent.id);
                    match a {
                        Some(id) => {
                            let success = schedule.dequeue(Box::new(*agent), *id);
                            //println!("La deschedulazione di {} ha avuto successo: {}",agent.id, success);
                            self.field1.remove_object_location(*agent, agent.loc);
                            self.field1.scheduled_agent.remove(&agent.id);
                        },
                        None => {},
                    }
                }        
            } 
        }

        for v in &vec{
            for bird in v{
                //println!("Schedulo {}", bird);
                let (counting, _) = schedule.distributed_schedule_repeating(Box::new(*bird), schedule.time + 1., 0);
                self.field1.scheduled_agent.insert(bird.id, counting);
                self.field1.insert(*bird, bird.loc);

            }
        }

        /* let events = schedule.get_all_events();
        for agent in &events{
            println!("Trovato agente {} negli eventi", agent.downcast_ref::<Bird>().unwrap());
        } */

        
        //println!("Finisco after_step");
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
