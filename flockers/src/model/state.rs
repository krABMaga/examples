use crate::model::bird::Bird;
use crate::DISCRETIZATION;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::kdtree_mpi::Kdtree;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::mpi::point_to_point::Destination;
use krabmaga::mpi::point_to_point::Source;
use krabmaga::mpi::topology::Communicator;
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
        self.field1 = Kdtree::create_tree(0, 0.0, 0.0, self.dim.0, self.dim.1, DISCRETIZATION, 25.)
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let world = universe.world();
        let mut rng = rand::thread_rng();

        if world.rank() == 0 {
            let mut vec: Vec<Vec<Bird>> = Vec::new();
            
            for _ in 0..world.size() {
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
                let bird = Bird::new(bird_id, loc, last_d);
                let id = self.field1.get_block_by_location(loc.x, loc.y);

                if id > 0 {
                    vec[(id) as usize].push(bird);
                } else {
                    self.field1.insert(bird, loc);
                    let (counting, _) =
                        schedule.distributed_schedule_repeating(Box::new(bird), 0., 0);
                    self.field1.scheduled_agent.insert(bird.id, counting);
                }
                if bird_id == self.initial_flockers - 1 {
                    for i in 1..world.size() {
                        world.process_at_rank(i).send(&vec[(i) as usize]);
                    }
                }
            }
        } else {
            let (vec, _) = world.process_at_rank(0).receive_vec::<Bird>();
            for bird in vec.iter() {
                self.field1.insert(*bird, bird.loc);
                let (counting, _) = schedule.distributed_schedule_repeating(Box::new(*bird), 0., 0);
                self.field1.scheduled_agent.insert(bird.id, counting);
            }
        }
    }

    fn update(&mut self, _step: u64) {
        self.field1.lazy_update();
    }

    fn before_step(&mut self, _: &mut Schedule) {
        let dummy = Bird {
            id: 0,
            loc: Real2D { x: 0., y: 0. },
            last_d: Real2D { x: 0., y: 0. },
        };

        if self.field1.received_neighbors.len() == 0 {
            let neighbors: Vec<Bird> = self
                .field1
                .message_exchange(&self.field1.prec_neighbors, dummy, true)
                .into_iter()
                .flatten()
                .collect();
            for agent in &neighbors {
                self.field1.insert_read(*agent, agent.loc);
            }
            self.field1.received_neighbors = neighbors;
        }
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        let dummy = Bird {
            id: 0,
            loc: Real2D { x: 0., y: 0. },
            last_d: Real2D { x: 0., y: 0. },
        };
        let vec = self
            .field1
            .message_exchange(&self.field1.agents_to_send, dummy, false);

        for vec in &self.field1.agents_to_send {
            if vec.len() != 0 {
                for agent in vec {
                    let a = self.field1.scheduled_agent.get(&agent.id);
                    match a {
                        Some(id) => {
                            schedule.dequeue(Box::new(*agent), *id);
                            self.field1.remove_object_location(*agent, agent.loc);
                            self.field1.scheduled_agent.remove(&agent.id);
                        }
                        None => {}
                    }
                }
            }
        }

        for v in &vec {
            for bird in v {
                let (counting, _) =
                    schedule.distributed_schedule_repeating(Box::new(*bird), schedule.time + 1., 0);
                self.field1.scheduled_agent.insert(bird.id, counting);
                self.field1.insert(*bird, bird.loc);
            }
        }
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
