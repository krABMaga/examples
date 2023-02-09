extern crate core;

use model::robot_factory::RobotFactory;

mod model;
mod visualization;

pub const ROBOT_COUNT: usize = 5;

pub const CHARGE_PER_STEP: i32 = 5;
pub const MAX_CHARGE: u32 = 350;
pub const INITIAL_CHARGE: i32 = 350;
pub const JUST_IN_TIME_CHARGE: i32 = 150;
pub const ENERGY_COST_PER_STEP: i32 = 1;
pub const ENERGY_COST_PER_STEP_WHILE_CARRYING: i32 = 2;

pub const FACTORY_WIDTH: f32 = 27.0;
pub const FACTORY_HEIGHT: f32 = 27.0;

pub const STEP: u64 = 500;

pub const ORDER_GENEREATION_CHANCE: f64 = 0.03;

pub const DELUXE_FINISHER_CYCLES: u32 = 7;
pub const STANDARD_FINISHER_CYCLES: u32 = 4;

pub const INITIAL_LOADING_DOCK_PRODUCTS: u32 = 10;

// No visualization specific imports
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let factory = RobotFactory::new();
    simulate!(factory, STEP, 1);
}
