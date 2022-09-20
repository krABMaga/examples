use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_3d::{toroidal_distance, toroidal_transform, Location3D};
use krabmaga::engine::location::Real3D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

use crate::model::state::Flocker;
use crate::{AVOIDANCE, COHESION, CONSISTENCY, JUMP, MOMENTUM, RANDOMNESS};

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u32,
    pub loc: Real3D,
    pub last_d: Real3D,
}

impl Bird {
    pub fn new(id: u32, loc: Real3D, last_d: Real3D) -> Self {
        Bird { id, loc, last_d }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Flocker>().unwrap();
        let vec = state.field1.get_neighbors_within_relax_distance(self.loc, 2.0);

        let width = state.dim.0;
        let height = state.dim.1;
        let length = state.dim.2;
        
        // let loc = Real3D { x: self.loc.x + 1.0, y: self.loc.y + 1.0 , z: self.loc.z + 1.0 };
        // println!("id = {}, loc = {}", self.id, loc);

        // for elem in &vec {
        //     println!("elem = {}", elem);
        // }
        // self.loc = loc;
        // drop(vec);
        // state
        //     .field1
        //     .set_object_location(*self, loc);

        let mut avoidance = Real3D { x: 0.0, y: 0.0, z: 0.0 };

        let mut cohesion = Real3D { x: 0.0, y: 0.0, z: 0.0 };
        let mut randomness = Real3D { x: 0.0, y: 0.0, z: 0.0 };
        let mut consistency = Real3D { x: 0.0, y: 0.0, z: 0.0 };

        if !vec.is_empty() {
            //avoidance
            let mut x_avoid = 0.0;
            let mut y_avoid = 0.0;
            let mut z_avoid = 0.0;
            let mut x_cohe = 0.0;
            let mut y_cohe = 0.0;
            let mut z_cohe = 0.0;
            let mut x_cons = 0.0;
            let mut y_cons = 0.0;
            let mut z_cons = 0.0;
            let mut count = 0;

            for elem in &vec {
                if self.id != elem.id {
                    let dx = toroidal_distance(self.loc.x, elem.loc.x, width);
                    let dy = toroidal_distance(self.loc.y, elem.loc.y, height);
                    let dz = toroidal_distance(self.loc.z, elem.loc.z, length);
                    count += 1;

                    //avoidance calculation
                    let square = dx * dx + dy * dy + dz * dz;
                    x_avoid += dx / (square * square + 1.0);
                    y_avoid += dy / (square * square + 1.0);
                    z_avoid += dz / (square * square + 1.0);

                    //cohesion calculation
                    x_cohe += dx;
                    y_cohe += dy;
                    z_cohe += dz;

                    //consistency calculation
                    x_cons += elem.last_d.x;
                    y_cons += elem.last_d.y;
                    z_cons += elem.last_d.z;
                }
            }

            if count > 0 {
                x_avoid /= count as f32;
                y_avoid /= count as f32;
                z_avoid /= count as f32;
                x_cohe /= count as f32;
                y_cohe /= count as f32;
                z_cohe /= count as f32;
                x_cons /= count as f32;
                y_cons /= count as f32;
                z_cons /= count as f32;

                consistency = Real3D {
                    x: x_cons / count as f32,
                    y: y_cons / count as f32,
                    z: z_cons / count as f32,
                };
            } else {
                consistency = Real3D {
                    x: x_cons,
                    y: y_cons,
                    z: z_cons,
                };
            }

            avoidance = Real3D {
                x: 400.0 * x_avoid,
                y: 400.0 * y_avoid,
                z: 400.0 * z_avoid,
            };

            cohesion = Real3D {
                x: -x_cohe / 10.0,
                y: -y_cohe / 10.0,
                z: -z_cohe / 10.0,
            };

            //randomness
            let mut rng = rand::thread_rng();
            let r1: f32 = rng.gen();
            let x_rand = r1 * 2.0 - 1.0;
            let r2: f32 = rng.gen();
            let y_rand = r2 * 2.0 - 1.0;
            let r3: f32 = rng.gen();
            let z_rand = r3 * 2.0 - 1.0;

            let square = (x_rand * x_rand + y_rand * y_rand + z_rand * z_rand).sqrt();
            randomness = Real3D {
                x: 0.05 * x_rand / square,
                y: 0.05 * y_rand / square,
                z: 0.05 * z_rand / square,
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
        let mut dz = COHESION * cohesion.z
            + AVOIDANCE * avoidance.z
            + CONSISTENCY * consistency.z
            + RANDOMNESS * randomness.z
            + MOMENTUM * mom.z;

        let dis = (dx * dx + dy * dy + dz * dz).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
            dz = dz / dis * JUMP;
        }

        self.last_d = Real3D { x: dx, y: dy, z: dz };

        let loc_x = toroidal_transform(self.loc.x + dx, width);
        let loc_y = toroidal_transform(self.loc.y + dy, height);
        let loc_z = toroidal_transform(self.loc.z + dz, length);

        self.loc = Real3D { x: loc_x, y: loc_y , z: loc_z };
        drop(vec);
        state
            .field1
            .set_object_location(*self, Real3D { x: loc_x, y: loc_y, z: loc_z });
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

impl Location3D<Real3D> for Bird {
    fn get_location(self) -> Real3D {
        self.loc
    }

    fn set_location(&mut self, loc: Real3D) {
        self.loc = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.id, self.loc)
    }
}
