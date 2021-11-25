use core::fmt;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::fields::field_2d::{toroidal_distance, toroidal_transform};
use rust_ab::engine::location::{Location2D, Real2D};
use rust_ab::engine::{schedule::Schedule, state::State};
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::hash::{Hash, Hasher};

use crate::model::state::Flocker;
use crate::{AVOIDANCE, COHESION, CONSISTENCY, JUMP, MOMENTUM, RANDOMNESS};

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u32,
    pub pos: Real2D,
    pub last_d: Real2D,
}

impl Bird {
    pub fn new(id: u32, pos: Real2D, last_d: Real2D) -> Self {
        Bird { id, pos, last_d }
    }

    // pub fn avoidance(self, vec: &[Bird], width: f32, height: f32) -> Real2D {
    //     if vec.is_empty() {
    //         let real = Real2D { x: 0.0, y: 0.0 };
    //         return real;
    //     }

    //     let mut x = 0.0;
    //     let mut y = 0.0;

    //     let mut count = 0;

    //     for elem in vec {
    //         if self != *elem {
    //             let dx = toroidal_distance(self.pos.x, elem.pos.x, width);
    //             let dy = toroidal_distance(self.pos.y, elem.pos.y, height);
    //             let square = dx * dx + dy * dy;
    //             count += 1;
    //             x += dx / (square * square + 1.0);
    //             y += dy / (square * square + 1.0);
    //         }
    //     }
    //     if count > 0 {
    //         x /= count as f32;
    //         y /= count as f32;
    //     }

    //     Real2D {
    //         x: 400.0 * x,
    //         y: 400.0 * y,
    //     }
    // }

    // pub fn cohesion(self, vec: &[Bird], width: f32, height: f32) -> Real2D {
    //     if vec.is_empty() {
    //         let real = Real2D { x: 0.0, y: 0.0 };
    //         return real;
    //     }

    //     let mut x = 0.0;
    //     let mut y = 0.0;

    //     let mut count = 0;

    //     for elem in vec {
    //         if self != *elem {
    //             let dx = toroidal_distance(self.pos.x, elem.pos.x, width);
    //             let dy = toroidal_distance(self.pos.y, elem.pos.y, height);
    //             count += 1;
    //             x += dx;
    //             y += dy;
    //         }
    //     }
    //     if count > 0 {
    //         x /= count as f32;
    //         y /= count as f32;
    //     }
    //     Real2D {
    //         x: -x / 10.0,
    //         y: -y / 10.0,
    //     }
    // }

    // pub fn randomness(self) -> Real2D {
    //     let mut rng = rand::thread_rng();
    //     let r1: f32 = rng.gen();
    //     let x = r1 * 2.0 - 1.0;
    //     let r2: f32 = rng.gen();
    //     let y = r2 * 2.0 - 1.0;

    //     let square = (x * x + y * y).sqrt();
    //     Real2D {
    //         x: 0.05 * x / square,
    //         y: 0.05 * y / square,
    //     }
    // }

    pub fn consistency(self, vec: &[Bird], width: f32, height: f32) -> Real2D {
        if vec.is_empty() {
            let real = Real2D { x: 0.0, y: 0.0 };
            return real;
        }

        let mut x = 0.0;
        let mut y = 0.0;

        let mut count = 0;

        for elem in vec {
            if self != *elem {
                /* let _dx = toroidal_distance(self.pos.x, vec[i].pos.x, width);
                let _dy = toroidal_distance(self.pos.y, vec[i].pos.y, height); */
                count += 1;
                x += elem.last_d.x;
                y += elem.last_d.y;
            }
        }
        if count > 0 {
            x /= count as f32;
            y /= count as f32;
            Real2D {
                x: x / count as f32,
                y: y / count as f32,
            }
        } else {
            Real2D { x, y }
        }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State, _schedule: &mut Schedule, _schedule_id: u32) {
        let state = state.as_any().downcast_ref::<Flocker>().unwrap();
        let vec = state.field1.get_neighbors_within_distance(self.pos, 10.0);

        let width = state.dim.0;
        let height = state.dim.1;

        //let avoid = self.avoidance(&vec, width, height);
        // let cohe = self.cohesion(&vec, width, height);
        // let rand = self.randomness();
        // let cons = self.consistency(&vec, width, height);

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

            for elem in &vec {
                if self.id != elem.id {
                    let dx = toroidal_distance(self.pos.x, elem.pos.x, width);
                    let dy = toroidal_distance(self.pos.y, elem.pos.y, height);
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
        let loc_x = toroidal_transform(self.pos.x + dx, width);
        let loc_y = toroidal_transform(self.pos.y + dy, width);

        self.pos = Real2D { x: loc_x, y: loc_y };
        drop(vec);
        state
            .field1
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }

    fn get_id(&self) -> u32 {
        self.id
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
        self.pos
    }

    fn set_location(&mut self, loc: Real2D) {
        self.pos = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} pos {}", self.id, self.pos)
    }
}
