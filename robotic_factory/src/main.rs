use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;

use model::robot_factory::*;
use model::stations::*;

mod model;
mod visualization;

fn main() {
    println!("Hello, world!");

    let mut factory = RobotFactory::new();

    let mut schedule = Schedule::new();

    factory.init(&mut schedule);

    let station = factory.get_random_station_with_type(StationType::RobotRoom);

    let robots = factory.get_robots();

    factory.get_robots().iter_mut().for_each(|x| {
        let mut robot = x.borrow_mut();
        robot.charge(5, &factory);
        println!("Robot {} has {} charge", robot.get_id(), robot.get_charge());
    });


    print!("Done!")
}
