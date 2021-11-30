use crate::model::bird::Bird;
use crate::{DISCRETIZATION, TOROIDAL};
use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::field_2d::Field2D;
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::rand;
use rust_ab::rand::Rng;
use std::any::Any;

pub struct Flocker {
    pub step: u64,
    pub field1: Field2D<Bird>,
    pub initial_flockers: u32,
    pub width: f32,
    pub height: f32,
}

impl Flocker {
    #[allow(dead_code)]
    //pub fn new(dim: (f32, f32), initial_flockers: u32) -> Self {
    pub fn new(width: f32, height: f32, initial_flockers: u32) -> Self {
        Flocker {
            step: 0,
            field1: Field2D::new(width, height, DISCRETIZATION, TOROIDAL),
            initial_flockers,
            width,
            height,
        }
    }
}

impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        //self.field1 = Field2D::new(self.width, self.height, DISCRETIZATION, TOROIDAL);
        self.field1 = Field2D::new(self.width, self.height, DISCRETIZATION, TOROIDAL);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut rng = rand::thread_rng();
        // Should be moved in the init method on the model exploration changes
        for bird_id in 0..self.initial_flockers {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let pos = Real2D {
                x: self.width * r1,
                y: self.height * r2,
            };
            let bird = Bird::new(bird_id, pos, last_d);
            self.field1.set_object_location(bird, pos);
            schedule.schedule_repeating(Box::new(bird), 0., 0);
        }
    }

    fn update(&mut self, _step: u64) {
        self.field1.lazy_update();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
}
