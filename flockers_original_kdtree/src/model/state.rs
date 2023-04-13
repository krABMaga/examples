use crate::model::bird::Bird;
use crate::{DISCRETIZATION, TOROIDAL};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::kdtree::Kdtree;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
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
            field1: Kdtree::create_tree(0, 0.0, 0.0, dim.0, dim.1),
            initial_flockers,
            dim,
        }
    }
}

impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Kdtree::create_tree(0, 0.0, 0.0, self.dim.0, self.dim.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut rng = rand::thread_rng();
        // Should be moved in the init method on the model exploration changes
        for bird_id in 0..self.initial_flockers {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let loc = Real2D {
                x: self.dim.0 * r1,
                y: self.dim.1 * r2,
            };
            let bird = Bird::new(bird_id, loc, last_d);
            self.field1.insert(bird, loc.x, loc.y);
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