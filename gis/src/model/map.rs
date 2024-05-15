use super::person::Person;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::Field2D;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::{
    engine::{schedule::Schedule, state::State},
    rand::{self, Rng},
};

pub struct Map {
    pub step: u64,
    pub field: Field2D<Person>,
    pub gis_field: DenseNumberGrid2D<i32>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl Map {
    pub fn new(dim: (f32, f32), num_agents: u32) -> Map {
        Map {
            step: 0,
            field: Field2D::new(dim.0, dim.1, 1.0, false),
            gis_field: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
            dim,
            num_agents,
        }
    }
}

impl State for Map {
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
        println!("{:?}", self.gis_field.locs.is_empty());
    }

    fn reset(&mut self) {
        self.step = 0;
        //self.field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1
    }

    fn set_gis(&mut self, vec: Vec<i32>) {
        let mut done = true;

        for (index, i) in vec.iter().enumerate() {
            let x = index as f32 / 30 as f32;
            let y = index as f32 % 30 as f32;
            self.gis_field.set_value_location(
                *i,
                &Int2D {
                    x: x as i32,
                    y: y as i32,
                },
            );

            if *i == 1 {
                println!("{:?} {:?}", x, y);
            }

            if *i == 1 && done {
                let last_d = Real2D { x, y };
                let loc = Real2D { x, y };
                let agent = Person {
                    id: 0 as u32,
                    loc,
                    last_d,
                    dir_x: 1.0,
                    dir_y: 1.0,
                };
                // Put the agent in your state
                self.field.set_object_location(agent, loc);
                done = false;
            }
        }
    }
}
