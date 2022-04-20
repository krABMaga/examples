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
use rust_ab::*;

pub struct Flocker {
    pub step: u64,
    pub field1: Field2D<Bird>,
    pub initial_flockers: u32,
    pub dim: (f32, f32),
}

impl Flocker {
    #[allow(dead_code)]
    pub fn new(dim: (f32, f32), initial_flockers: u32) -> Self {
        Flocker {
            step: 0,
            field1: Field2D::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL),
            initial_flockers,
            dim,
        }
    }
}

impl State for Flocker {
    fn reset(&mut self) {
        self.step = 0;
        self.field1 = Field2D::new(self.dim.0, self.dim.1, DISCRETIZATION, TOROIDAL);
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
            self.field1.set_object_location(bird, loc);
            schedule.schedule_repeating(Box::new(bird), 0., 0);
        }
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        if schedule.step == 1 {

            addplot!(String::from("chart1"), String::from("xxxx"), String::from("yyyyy"));
            addplot!(String::from("chart2"), String::from("xxxx"), String::from("yyyyy"));
            addplot!(String::from("chart3"), String::from("xxxx"), String::from("yyyyy"));

            plot!(String::from("chart1"), String::from("s1"), 0.0, 10.0);
            plot!(String::from("chart1"), String::from("s1"), 10.0, 20.0);
            plot!(String::from("chart1"), String::from("s1"), 20.0, 30.0);

            plot!(String::from("chart1"), String::from("s2"), 30.0, 40.0);
            plot!(String::from("chart1"), String::from("s2"), 40.0, 50.0);
            plot!(String::from("chart1"), String::from("s2"), 50.0, 60.0);

            plot!(String::from("chart2"), String::from("s3"), 1.0, 1.0);
            plot!(String::from("chart3"), String::from("ss"), 1.0, 1.0);
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
