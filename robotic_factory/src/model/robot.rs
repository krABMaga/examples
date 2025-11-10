use std::cmp::min;
use std::fmt;
use std::hash::Hash;

use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::*;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::ScheduleOptions;
use krabmaga::engine::state::State;
use krabmaga::rand::seq::IteratorRandom;
use krabmaga::*;

use StationType::*;

use crate::model::robot_factory::{RobotFactory, StationLocation};
use crate::model::stations::{Station, StationType};
use crate::{
    ENERGY_COST_PER_STEP, ENERGY_COST_PER_STEP_WHILE_CARRYING, INITIAL_CHARGE, MAX_CHARGE,
};

//----------------Robot----------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CarriedProduct {
    Nothing,
    Bolts,
    Cuttings,
    StichedStandard,
    StichedDeluxe,
    FinishedStandard,
    FinishedDeluxe,
}

impl CarriedProduct {
    fn get_destination_type_of_product(&mut self) -> StationType {
        match self {
            CarriedProduct::Nothing => LoadingDock,
            CarriedProduct::Bolts => Cutter,
            CarriedProduct::Cuttings => Sticher,
            CarriedProduct::StichedStandard | CarriedProduct::StichedDeluxe => Finisher,
            CarriedProduct::FinishedStandard | CarriedProduct::FinishedDeluxe => LoadingDock,
        }
    }
}

#[derive(Clone, Copy, Eq, Debug)]
pub struct Robot {
    id: usize,
    pub max_charge: u32,
    pub charge: i32,
    location: Real2D,
    next_location: Real2D,
    // marks where to find the robot in the next step
    destination: Real2D,
    destination_type: StationType,
    order: CarriedProduct,
}

impl Robot {
    pub fn change_destination(&mut self, target: StationLocation) {
        self.destination = target.location;
        self.destination_type = target.station_type;
        self.update_next_location();
        log!(
            LogType::Info,
            format!(
                "Robot {} changed destination to a {:?}(at {})",
                self.id, self.destination_type, self.destination
            ),
            true
        );
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_next_location(&self) -> Real2D {
        self.next_location
    }
}

impl Location2D<Real2D> for Robot {
    fn get_location(self) -> Real2D {
        self.location
    }

    fn set_location(&mut self, location: Real2D) {
        self.location = location;
    }
}

impl Hash for Robot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Robot(id: {} at {}, dest {}, carries {:?})",
            self.id, self.location, self.destination, self.order
        )
    }
}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Robot {
    pub fn new(id: usize, location: Real2D, state: &dyn State) -> Robot {
        let robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
        let initial_destination = robot_factory
            .get_random_station_location_with_type(LoadingDock)
            .location;
        let mut robot = Robot {
            id,
            max_charge: MAX_CHARGE,
            charge: INITIAL_CHARGE,
            location,
            next_location: location,
            destination: initial_destination,
            destination_type: LoadingDock,
            order: CarriedProduct::Nothing,
        };
        robot.update_next_location();
        robot
    }

    pub fn charge(&mut self, amount: i32, state: &RobotFactory) {
        self.charge = min(self.max_charge as i32, self.charge + amount);
        self.order = CarriedProduct::Nothing;

        if self.is_fully_charged() {
            if rand::thread_rng().gen_bool(0.5) {
                self.change_destination(state.get_random_station_location());
            } else {
                self.change_destination(state.get_random_station_location_with_type(LoadingDock));
            }
        }
    }

    pub fn is_fully_charged(&self) -> bool {
        self.charge == self.max_charge as i32
    }

    fn update_next_location(&mut self) {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;

        //ensure max step distance is 1 by normalization
        let distance = (dx * dx + dy * dy).sqrt();
        if distance > 1.0 {
            dx /= distance;
            dy /= distance;
        }

        //apply direction vector to current position
        self.next_location = Real2D {
            x: x + dx,
            y: y + dy,
        };
    }

    fn move_step_towards_destination(&mut self) {
        log!(
            LogType::Info,
            format!(
                "Robot {} moving from ({}) to ({})",
                self.id, self.location, self.next_location
            ),
            true
        );

        self.location = self.next_location;
        self.update_next_location();

        if self.order == CarriedProduct::Nothing {
            self.charge -= ENERGY_COST_PER_STEP;
        } else {
            self.charge -= ENERGY_COST_PER_STEP_WHILE_CARRYING;
        }
    }

    fn is_at_destination(&self) -> bool {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let dx = self.destination.x - x;
        let dy = self.destination.y - y;

        let distance_sq = (dx * dx + dy * dy).abs();
        return distance_sq == 0.0; //todo: maybe we need to check with a small epsilon
    }

    fn renew_destination(&mut self, factory: &RobotFactory) {
        self.destination_type = self.order.get_destination_type_of_product();

        if self.destination_type == Finisher {
            let finishers = factory.get_stations_of_type(self.destination_type);
            let finisher = finishers
                .iter()
                .filter(|station| {
                    station.is_deluxe_finisher() == (self.order == CarriedProduct::StichedDeluxe)
                })
                .choose(&mut rand::thread_rng());

            self.change_destination(
                finisher
                    .expect(&*format!(
                        "No finisher found for order type {:?}!",
                        self.order
                    ))
                    .to_station_location(),
            );
        } else {
            self.change_destination(
                factory.get_random_station_location_with_type(self.destination_type),
            );
        }
    }

    fn drop_off_product(&mut self, station: &mut Station) {
        self.order = CarriedProduct::Nothing;
        station.material_management.increment_supply();
    }
}

impl Agent for Robot {
    fn step(&mut self, state: &mut dyn State) {
        //Set robot destination

        let mut robot_factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

        if !self.is_at_destination() {
            self.move_step_towards_destination();
        } else {
            let mut neighbor_stations = robot_factory
                .station_grid
                .get_neighbors_within_distance(self.location, 0.01);

            //All (normal) stations write their state into the buffer beforehand.
            //Since multiple robots may manipulate the stations we need to get the write information.
            let mut neighbours: Vec<_> = neighbor_stations
                .iter_mut()
                .flat_map(|station| {
                    robot_factory
                        .station_grid
                        .get_objects_unbuffered(station.get_location())
                })
                .filter(|station| station.get_station_type() == self.destination_type)
                .collect();

            //however, the LoadingDock is executed later, so it may needs to be fetched from the read buffer
            if neighbours.is_empty() {
                neighbours = neighbor_stations
                    .iter_mut()
                    .flat_map(|station| {
                        robot_factory
                            .station_grid
                            .get_objects(station.get_location())
                    })
                    .filter(|station| station.get_station_type() == self.destination_type)
                    .collect();
            }

            let station_opt = neighbours.iter_mut().choose(&mut rand::thread_rng());

            if station_opt.is_none() {
                let msg = format!(
                    "Robot {} is at destination {} but no station of type {:?} found",
                    self.id, self.destination, self.destination_type
                );
                log!(LogType::Error, msg.clone(), true);
                panic!("{}", msg);
            }

            let mut station = station_opt.unwrap();

            match self.destination_type {
                LoadingDock => {
                    if station.has_product_available() {
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    }
                }
                Sticher | Cutter | Finisher => {
                    self.drop_off_product(&mut station);
                    if station.has_product_available() {
                        //todo: this should check all stations around the robot
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    } else {
                        self.change_destination(
                            robot_factory.get_random_station_location_with_type(LoadingDock),
                        );
                    }
                }
                StorageRoom => {
                    self.drop_off_product(&mut station);
                    self.order = station.take_product(&mut robot_factory); //storage room returns CarriedProduct::Nothing
                    self.renew_destination(robot_factory);
                }
                RobotRoom => {
                    //nothing to-do here
                }
            }

            //update station on the grid
            robot_factory
                .station_grid
                .remove_object_location(*station, station.get_location());
            robot_factory
                .station_grid
                .set_object_location(*station, station.get_location());
        }

        //return-home
        if self.charge < 2 && self.destination_type != RobotRoom {
            let loading_station = robot_factory.get_random_station_location_with_type(RobotRoom);
            self.change_destination(loading_station);
        }

        robot_factory
            .robot_grid
            .set_object_location(*self, self.location);
    }

    fn before_step(
        &mut self,
        state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        //load self state from factory state
        let factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

        let mut robots = factory
            .robot_grid
            .get_objects_unbuffered(self.next_location); //in the buffer, the robot is already at the new location
        for robot in robots.iter() {
            if robot.id == self.id {
                self.order = robot.order;
                self.destination = robot.destination;
                self.destination_type = robot.destination_type;
                self.charge = robot.charge;
                self.next_location = robot.next_location;
                return None; //if we find an updated state of the robot, we update ourselves
            }
        }

        robots = factory.robot_grid.get_objects(self.location);
        for robot in robots.iter() {
            if robot.id == self.id {
                self.order = robot.order;
                self.destination = robot.destination;
                self.destination_type = robot.destination_type;
                self.charge = robot.charge;
                self.next_location = robot.next_location;
                break;
            }
        }
        None
    }

    fn after_step(&mut self, state: &mut dyn State) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        let factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();
        plot!(
                String::from("Robots' Energy"),
                format!("Robot {}", self.get_id()),
                factory.step as f64,
                (self.charge as f64 / self.max_charge as f64) * 100.0
            );

        None
    }
}

#[cfg(test)]
mod tests {
    use krabmaga::engine::fields::field::Field;
    use krabmaga::engine::fields::field_2d::Field2D;
    use krabmaga::engine::schedule::Schedule;
    use krabmaga::thread_rng;

    use super::*;

    #[test]
    fn does_charge() {
        let mut factory = RobotFactory::new();
        let mut scheduler = Schedule::new();
        factory.init(&mut scheduler);
        let mut robot = Robot::new(0, Real2D { x: 0.0, y: 0.0 }, factory.as_state());

        let original_charge = robot.charge;
        let charging_amount = thread_rng().gen_range(1..=10) as i32;

        //increase max charge to ensure we don't bump into the max charge
        robot.max_charge = (original_charge + charging_amount + 1) as u32;
        robot.charge(charging_amount, &factory);

        assert_eq!(robot.charge, charging_amount + original_charge);
    }

    #[test]
    fn does_not_overcharge() {
        let mut factory = RobotFactory::new();
        let mut scheduler = Schedule::new();
        factory.init(&mut scheduler);
        let mut robot = Robot::new(0, Real2D { x: 0.0, y: 0.0 }, factory.as_state());

        robot.max_charge = thread_rng().gen_range(10..=1000) as u32;
        robot.charge = robot.max_charge as i32; //fully charge the robot

        let charging_amount = thread_rng().gen_range(1..=10) as i32;
        robot.charge(charging_amount, &factory);

        assert_eq!(robot.charge as u32, robot.max_charge);
    }
}
