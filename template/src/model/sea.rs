use super::crab::Crab;
use crate::{DISCRETIZATION, HEIGHT, NUM_AGENTS, TOROIDAL, WIDTH};
use rust_ab::engine::fields::field::Field;
use rust_ab::{
    engine::{fields::field_2d::Field2D, location::Real2D, schedule::Schedule, state::State},
    rand::{self, Rng},
};

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' positions.
pub struct Sea {
    pub step: u64,
    pub field: Field2D<Crab>,
}

impl Sea {
    pub fn new() -> Sea {
        Sea {
            step: 0,
            field: Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL),
        }
    }
}

impl State for Sea {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
        self.field = Field2D::new(WIDTH, HEIGHT, DISCRETIZATION, TOROIDAL);
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut rust_ab::engine::schedule::Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();

        for i in 0..NUM_AGENTS {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let last_d = Real2D { x: 0., y: 0. };
            let pos = Real2D {
                x: WIDTH * r1,
                y: HEIGHT * r2,
            };
            let agent = Crab {
                id: i,
                pos,
                last_d,
                dir_x: 1.0,
                dir_y: 1.0,
            };
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
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
}
