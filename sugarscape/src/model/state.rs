use std::any::Any;

use crate::model::eater::Eater;
use core::fmt;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::{
    engine::{location::Int2D, schedule::Schedule, state::State},
    rand::Rng,
};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Patch {
    pub id: u32,
    pub sugar_amount: u32,
    sugar_growback: u32,
}

impl Patch {
    pub fn new(id: u32, sugar_amount: u32, _sugar_growback: u32) -> Self {
        Patch {
            id,
            sugar_amount,
            sugar_growback: 1,
        }
    }
}

impl Hash for Patch {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Patch {}

impl PartialEq for Patch {
    fn eq(&self, other: &Patch) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} value {}", self.id, self.sugar_amount)
    }
}

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct Environment {
    pub step: u64,
    pub field: DenseNumberGrid2D<Patch>,
    pub dim: (i32, i32),
    pub num_agents: u32,
    pub eaters: DenseGrid2D<Eater>,
}

impl Environment {
    pub fn new(dim: (i32, i32), num_agents: u32) -> Environment {
        Environment {
            step: 0,
            field: DenseNumberGrid2D::new(dim.0, dim.1),
            dim,
            num_agents,
            eaters: DenseGrid2D::new(dim.0, dim.1),
        }
    }
}

impl State for Environment {
    //At each step the positions of eaters into the field are updated.
    fn update(&mut self, step: u64) {
        if step == 0 {
            self.field.lazy_update();
        }
        self.eaters.lazy_update();
        self.step = step;
    }

    //Resets the state
    fn reset(&mut self) {
        self.step = 0;
        self.field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
        self.eaters = DenseGrid2D::new(self.dim.0, self.dim.1)
    }

    //Initializes all the patches and eaters
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
        let mut id = 0;
        let mut rng = krabmaga::rand::thread_rng();

        let bottom_left_mid = (
            (self.dim.0 as f32 * 0.25) as i32,
            (self.dim.1 as f32 * 0.25) as i32,
        );
        let up_right_mid = (
            (self.dim.0 as f32 * 0.75) as i32 - 1,
            (self.dim.1 as f32 * 0.75) as i32,
        );
        let top_left_corner = (0, self.dim.1);
        let bottom_right_corner = (self.dim.0, 0);
        let dim_div = (bottom_left_mid.0).pow(2);

        let dimen = bottom_left_mid.0 + bottom_left_mid.1;
        let dimen_up = up_right_mid.0 + up_right_mid.1;
        let dimen_corner_up = top_left_corner.1;
        let dimen_corner_down = bottom_right_corner.0;

        //Initializes the patches
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                let x = j - dimen / 2;
                let x_up = j - dimen_up / 2;
                let x_corner_up = j - dimen_corner_up;
                let x_corner_down = j;
                let y = dimen / 2 - i;
                let y_up = dimen_up / 2 - i;
                let y_corner_up = i;
                let y_corner_down = dimen_corner_down - i;

                let sumsq = x * x + y * y;
                let sumsq_up = x_up * x_up + y_up * y_up;
                let sumsq_corner = x_corner_up * x_corner_up + y_corner_up * y_corner_up;
                let sumsq_corner_down =
                    x_corner_down * x_corner_down + y_corner_down * y_corner_down;

                let mut sugar_amount = 1;

                if ((0 <= sumsq) && (sumsq <= dim_div))
                    || ((0 <= sumsq_up) && (sumsq_up <= dim_div))
                {
                    sugar_amount = 3
                } else if ((0 <= sumsq) && (sumsq <= dim_div * 2))
                    || ((0 <= sumsq_up) && (sumsq_up <= dim_div * 2))
                {
                    sugar_amount = 2;
                } else if (0 <= sumsq_corner) && (sumsq_corner <= dim_div)
                    || (0 <= sumsq_corner_down) && (sumsq_corner_down <= dim_div)
                {
                    sugar_amount = 0;
                }

                let pos = Int2D { x: i, y: j };
                let sugar_growback = rng.gen_range(0..4);
                let patch = Patch::new(id, sugar_amount, sugar_growback);
                id += 1;
                self.field.set_value_location(patch, &pos);
            }
        }

        //Initializes the Agents
        for i in 0..self.num_agents {
            let xx = rng.gen_range(0..self.dim.0);
            let yy = rng.gen_range(0..self.dim.1);
            let pos = Int2D { x: xx, y: yy };
            let agent = Eater {
                id: i,
                position: pos,
                vision: rng.gen_range(1..4),
                metabolism: rng.gen_range(1..4),
                age: 0,
                max_age: 20,
                wealth: 20,
            };

            self.eaters.set_object_location(agent, &pos);
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
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
    fn after_step(&mut self, _schedule: &mut Schedule) {}
}
