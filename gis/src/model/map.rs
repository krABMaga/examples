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

        let mut rng = rand::thread_rng();

        for i in 0..self.num_agents {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let loc = Real2D {
                x: (self.dim.0 * r1) as f32,
                y: (self.dim.1 * r2) as f32,
            };
            let agent = Person {
                id: i,
                loc,
                last_d,
                dir_x: 1.0,
                dir_y: 1.0,
            };
            // Put the agent in your state
            self.field.set_object_location(agent, loc);
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
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
        for (index, i) in vec.iter().enumerate() {
            self.gis_field.set_value_location(
                *i,
                &Int2D {
                    x: index as i32 % self.dim.0 as i32,
                    y: index as i32 / self.dim.0 as i32,
                },
            );
        }

        //volendo posso mettere qua la generazione dell'agente
    }
}
