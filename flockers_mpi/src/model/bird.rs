use cfg_if::cfg_if;
use krabmaga::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "distributed_mpi"))]
    {
        use core::fmt;
        use krabmaga::mpi;
        use krabmaga::offset_of;
        use mpi::{datatype::{UncommittedUserDatatype, UserDatatype}, Address};
        use krabmaga::Equivalence;
        use krabmaga::engine::agent::Agent;
        use krabmaga::engine::fields::kdtree_mpi::{toroidal_distance, toroidal_transform, Location2D};
        use krabmaga::engine::location::Real2D;
        use krabmaga::engine::state::State;
        use krabmaga::rand;
        use krabmaga::rand::Rng;
        use krabmaga::UNIVERSE;
        use mpi::topology::Communicator;
        use std::hash::{Hash, Hasher};

        use crate::model::state::Flocker;
        use crate::{AVOIDANCE, COHESION, CONSISTENCY, JUMP, MOMENTUM, RANDOMNESS};

        #[derive(Clone, Copy)]
        pub struct Bird {
            pub id: u32,
            pub loc: Real2D,
            pub last_d: Real2D,
        }

        unsafe impl Equivalence for Bird{
            type Out = UserDatatype;
            fn equivalent_datatype() -> Self::Out{
                let real_2d = UncommittedUserDatatype::structured(
                    &[1,1],
                    &[
                        offset_of!(Real2D,x) as Address,
                        offset_of!(Real2D,y) as Address,
                    ],
                    &[f32::equivalent_datatype(),f32::equivalent_datatype()]
                );

                UserDatatype::structured(
                    &[1,1,1],
                    &[
                        offset_of!(Bird,id) as Address,
                        offset_of!(Bird,loc) as Address,
                        offset_of!(Bird,last_d) as Address
                    ],
                    &[
                        u32::equivalent_datatype().into(),
                        real_2d.as_ref(),
                        real_2d.as_ref()
                    ]
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
                let state = state.as_any_mut().downcast_mut::<Flocker>().unwrap();

                let world = UNIVERSE.world();

                let vec = state
                    .field1
                    .get_distributed_neighbors_within_relax_distance(self.loc, 10.0);

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

                //Get the block id of the agent: if id is equal to the process handling this agent, insert it into the field
                //else, put the agent in the array of agents that need to be sent to their respecive process
                //Example: if this instance is executed on process 3 and the id is also 3, then put the agent into the field.
                let id = state.field1.get_block_by_location(self.loc.x, self.loc.y);
                if id as i32 == world.rank() {
                    state.field1.insert(*self, self.loc);
                } else {
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
    }
}
