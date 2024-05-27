use super::person::Person;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::Field2D;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::{schedule::Schedule, state::State};

pub struct Map {
    pub step: u64,
    pub field: Field2D<Person>,
    pub gis_field: DenseNumberGrid2D<i32>,
    pub dim: (f32, f32),
    pub num_agents: u32,
    pub people: Vec<Person>,
}

impl Map {
    pub fn new(dim: (f32, f32), num_agents: u32) -> Map {
        Map {
            step: 0,
            field: Field2D::new(dim.0, dim.1, 1.0, false),
            gis_field: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
            dim,
            num_agents,
            people: Vec::new(),
        }
    }

    pub fn gis_value(&self, loc: Real2D) -> i32 {
        return self
            .gis_field
            .get_value(&Int2D {
                x: loc.x as i32,
                y: loc.y as i32,
            })
            .unwrap();
    }
}

impl State for Map {
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        //self.field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
    }

    fn init(&mut self, _schedule: &mut Schedule) {
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

    fn set_gis(&mut self, vec: Vec<i32>, schedule: &mut Schedule) {
        let loc = Real2D { x: 15., y: 25. };
        let agent = Person {
            id: 0 as u32,
            loc,
            dir_x: 1.0,
            dir_y: 1.0,
            direction: None,
        };
        self.people.push(agent.clone());
        self.field.set_object_location(agent, loc);
        schedule.schedule_repeating(Box::new(agent), 0., 0);

        for (index, i) in vec.iter().enumerate() {
            let x = (index as f32 / 30.).floor();
            let y = index as f32 % 30.;

            self.gis_field.set_value_location(
                *i,
                &Int2D {
                    x: x as i32,
                    y: y as i32,
                },
            );
        }
        self.gis_field.lazy_update();
    }
}
