extern crate core;

use model::robot_factory::RobotFactory;

mod model;
mod visualization;

pub const ROBOT_COUNT: usize = 3;
pub const JUST_IN_TIME_CHARGE: u32 = 50;

pub const CHARGE_PER_STEP: i32 = 5;
pub const MAX_CHARGE: u32 = 350;
pub const INITIAL_CHARGE: i32 = 100;
pub const ENERGY_COST_PER_STEP: i32 = 1;
pub const ENERGY_COST_PER_STEP_WHILE_CARRYING: i32 = 2;

pub const FACTORY_WIDTH: f32 = 100.0;
pub const FACTORY_HEIGHT: f32 = 100.0;

pub const STEP: u64 = 100;

pub const ORDER_GENEREATION_CHANCE: f64 = 0.03;

pub const DELUXE_FINISHER_CYCLES: u32 = 7;
pub const STANDARD_FINISHER_CYCLES: u32 = 4;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let factory = RobotFactory::new();
    simulate!(factory, STEP, 1);
}


    factory.update(1);

    let derp = factory.robot_grid.get_neighbors_within_distance(Real2D { x: 0.0, y: 0.0 }, 2.0);

    for mut robot in derp {
        robot.charge(5, &factory);
    }

    factory.update(2);

    let robots = factory.get_robots();

    print!("Done!")
}
