use crate::model::bird::Bird;
use crate::{DISCRETIZATION, TOROIDAL};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_3d::Field3D;
use krabmaga::engine::location::Real3D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;

pub struct Flocker {
    pub step: u64,
    pub field1: Field3D<Bird>,
    pub initial_flockers: u32,
    pub dim: (f32, f32, f32),
}

impl Flocker {
    #[allow(dead_code)]
    pub fn new(dim: (f32, f32, f32), initial_flockers: u32) -> Self {
        Flocker {
            step: 0,
            field1: Field3D::new(dim.0, dim.1, dim.2, DISCRETIZATION, TOROIDAL),
            initial_flockers,
            dim,
        }
    }
}

impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Field3D::new(self.dim.0, self.dim.1, self.dim.2, DISCRETIZATION, TOROIDAL);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut rng = rand::thread_rng();
        // Should be moved in the init method on the model exploration changes
        for bird_id in 0..self.initial_flockers {
            // let r1: f32 = rng.gen();
            // let r2: f32 = rng.gen();
            // let r3: f32 = rng.gen();
            // let last_d = Real3D { x: 0., y: 0., z: 0. };
            // let loc = Real3D {
            //     x: self.dim.0 * r1,
            //     y: self.dim.1 * r2,
            //     z: self.dim.2 * r3,
            // };
            let last_d = Real3D { x: 0., y: 0., z: 0. };
            let loc = Real3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let bird = Bird::new(bird_id, loc, last_d);
            // println!("Bird {} created at {:?}", bird_id, bird.loc);
            self.field1.set_object_location(bird, loc);
            // println!("Bird {} added at {:?}", bird_id, bird.loc);
            schedule.schedule_repeating(Box::new(bird), 0., 0);
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
