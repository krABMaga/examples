use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;

use model::robot_factory::*;
use model::stations::*;

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

fn main() {
    println!("Hello, world!");

    let mut factory = RobotFactory::new();

    let mut schedule = Schedule::new();

    factory.init(&mut schedule);
    factory.update(0);


    let robot_room_location = factory.get_random_station_location_with_type(StationType::RobotRoom);

    let neighbors = factory.robot_grid.get_neighbors_within_distance(robot_room_location.location, 2.0);

    for mut neighbor_robot in neighbors {
        neighbor_robot.charge(5, &factory);
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
