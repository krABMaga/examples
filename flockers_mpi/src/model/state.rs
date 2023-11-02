use cfg_if::cfg_if;
use krabmaga::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "distributed_mpi"))]
    {
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
        use krabmaga::UNIVERSE;
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

            ///This function creates the initial agents of the simulation.
            ///The init function is executed by all processes, but only the process 0 creates the agents.
            ///For each new agent, their id will be calculated: if the id is not equal to 0, then the agent will be put into an array 'vec'.
            ///When all agents have been created, all the agents in 'vec' will be sent to their respective process.
            ///The other processes, instead, will be waiting to receive the agents from process 0.
            fn init(&mut self, schedule: &mut Schedule) {
                let world = UNIVERSE.world();
                let mut rng = rand::thread_rng();

                //Process 0 creates the agents
                if world.rank() == 0 {
                    let mut vec: Vec<Vec<Bird>> = Vec::new();

                    //Create 'vec' with size equal to the number of processes
                    for _ in 0..world.size() {
                        vec.push(vec![])
                    }

                    //For each initial agent...
                    for bird_id in 0..self.initial_flockers {
                        let r1: f32 = rng.gen();
                        let r2: f32 = rng.gen();
                        let last_d = Real2D { x: 0., y: 0. };
                        let loc = Real2D {
                            x: self.dim.0 * r1,
                            y: self.dim.1 * r2,
                        };
                        let bird = Bird::new(bird_id, loc, last_d);
                        //Calculate its 'id'...
                        let id = self.field1.get_block_by_location(loc.x, loc.y);

                        //If 'id' is not 0, push the agent into 'vec' at position 'id'
                        //else, schedule the agent
                        if id > 0 {
                            vec[(id) as usize].push(bird);
                        } else {
                            self.field1.insert(bird, loc);
                            let (counting, _) =
                                schedule.distributed_schedule_repeating(Box::new(bird), 0., 0);
                            self.field1.scheduled_agent.insert(bird.id, counting);
                        }
                        //Once all agents have been created, send them to their respective process.
                        if bird_id == self.initial_flockers - 1 {
                            for i in 1..world.size() {
                                world.process_at_rank(i).send(&vec[(i) as usize]);
                            }
                        }
                    }
                } else {
                    //All other processes receive the agents
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

            ///The before_step function takes action before the start of the step.
            ///In this function, the agents sent from the other processes, for neighborhood calculation, will be received.
            ///These agent will be inserted into the field in 'read' mode.
            ///This will make them visible to the other agents in the field in order to calculate their neighborhood.
            fn before_step(&mut self, _: &mut Schedule) {
                let dummy = Bird {
                    id: 0,
                    loc: Real2D { x: 0., y: 0. },
                    last_d: Real2D { x: 0., y: 0. },
                };

                //Get the sent agents and read-insert them into the field.
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

            ///The after_step function takes action after the end of the step.
            ///In this function, the agents that must be sent to their respective processes are sent.
            ///Also, the agents that must be sent will be removed from the field and descheduled.
            ///Then, the agents received in the message_exchange phase will be inserted into the field and scheduled.
            fn after_step(&mut self, schedule: &mut Schedule) {
                let dummy = Bird {
                    id: 0,
                    loc: Real2D { x: 0., y: 0. },
                    last_d: Real2D { x: 0., y: 0. },
                };
                let vec = self
                    .field1
                    .message_exchange(&self.field1.agents_to_send, dummy, false);

                //All the agents that have to be sent are removed from the field and descheduled
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

                //All the agents that have been received must be inserted into the field and scheduled
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
    }
}
