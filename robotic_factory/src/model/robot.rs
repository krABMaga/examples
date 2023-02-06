use std::cmp::min;
use std::fmt;
use std::hash::Hash;

use krabmaga::{rand, Rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand::seq::IteratorRandom;

use crate::{ENERGY_COST_PER_STEP, ENERGY_COST_PER_STEP_WHILE_CARRYING, INITIAL_CHARGE, MAX_CHARGE};
use crate::model::robot_factory::{RobotFactory, StationLocation};
use crate::model::stations::{Station, StationType};

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
            CarriedProduct::Nothing => StationType::LoadingDock,
            CarriedProduct::Bolts => StationType::Cutter,
            CarriedProduct::Cuttings => StationType::Sticher,
            CarriedProduct::StichedStandard | CarriedProduct::StichedDeluxe => StationType::Finisher,
            CarriedProduct::FinishedStandard | CarriedProduct::FinishedDeluxe => StationType::LoadingDock,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Robot {
    pub id: u32,
    pub max_charge: u32,
    pub charge: i32,
    location: Real2D,
    destination: Real2D,
    destination_type: StationType,
    order: CarriedProduct,
}


impl Robot {
    pub fn change_destination(&mut self, target: StationLocation) {
        self.destination = target.location;
        self.destination_type = target.station_type;
    }

    pub fn get_charge(&self) -> i32 {
        self.charge
    }

    pub fn get_max_charge(&self) -> u32 {
        self.max_charge
    }

    pub fn set_order(&mut self, order: CarriedProduct) {
        self.order = order;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_id_mut(&mut self) -> u32 {
        self.id
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
        write!(f, "Robot(id: {} at {}, dest {}, carries {:?})", self.id, self.location, self.destination, self.order)
    }
}

impl Robot {
    pub fn new(id: u32, location: Real2D, state: &dyn State) -> Robot {
        let robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
        let initial_destination = robot_factory.get_random_station_location_with_type(StationType::LoadingDock).location;
        Robot {
            id,
            max_charge: MAX_CHARGE,
            charge: INITIAL_CHARGE,
            location,
            destination: initial_destination,
            destination_type: StationType::LoadingDock,
            order: CarriedProduct::Nothing,
        }
    }


    pub fn charge(&mut self, amount: i32, state: &RobotFactory) {
        self.charge = min(self.max_charge as i32, self.charge + amount);
        self.order = CarriedProduct::Nothing;

        if self.is_fully_charged() {
            if rand::thread_rng().gen_bool(0.5) {
                self.change_destination(state.get_random_station_location());
            } else {
                self.change_destination(state.get_random_station_location_with_type(StationType::LoadingDock));
            }
        }

        state.robot_grid.set_object_location(*self, self.location);
    }

    pub fn is_fully_charged(&self) -> bool {
        self.charge == self.max_charge as i32
    }


    fn move_step_towards_destination(&mut self, state: &RobotFactory) {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;

        //get length of direction vector

        //ensure max step disatance is 1 by normalization
        let distance = (dx * dx + dy * dy).sqrt();
        if distance > 1.0 {
            dx /= distance;
            dy /= distance;
        }

        //apply direction vector to current position
        self.location = Real2D { x: x + dx, y: y + dy };
        state.robot_grid.set_object_location(*self, self.location);

        if self.order == CarriedProduct::Nothing {
            self.charge -= ENERGY_COST_PER_STEP;
        } else {
            self.charge -= ENERGY_COST_PER_STEP_WHILE_CARRYING;
        }
    }

    fn is_at_destination(&self, state: &RobotFactory) -> bool {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;

        let distance_sq = (dx * dx + dy * dy);
        return distance_sq < 0.01;
    }

    fn renew_destination(&mut self, state: &RobotFactory) {
        self.destination_type = self.order.get_destination_type_of_product();
        self.change_destination(state.get_random_station_location_with_type(self.destination_type));
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

        if !self.is_at_destination(robot_factory) {
            self.move_step_towards_destination(robot_factory);
        } else {
            let mut neighbor_stations = robot_factory.station_grid.get_neighbors_within_distance(self.location, 0.01);
            let mut station_opt = neighbor_stations.iter_mut()
                .filter(|station| station.get_station_type() == self.destination_type)
                .choose(&mut rand::thread_rng());

            if station_opt.is_none() {
                panic!("Robot {} is at destination but no station of type {:?} found", self.id, self.destination_type);
            }

            let mut station = station_opt.unwrap();

            match self.destination_type {
                StationType::LoadingDock => {
                    if station.has_product_available() {
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    }
                }
                StationType::Sticher | StationType::Cutter | StationType::Finisher => {
                    self.drop_off_product(&mut station);
                    if station.has_product_available() { //todo: this should check all stations around the robot
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    } else {
                        self.change_destination(robot_factory.get_random_station_location_with_type(StationType::LoadingDock));
                    }
                }
                StationType::StorageRoom => {
                    self.drop_off_product(&mut station);
                    self.order = station.take_product(&mut robot_factory); //storage room returns CarriedProduct::Nothing
                    self.renew_destination(robot_factory);
                }
                StationType::RobotRoom => {
                    //nothing to-do here
                }
            }
        }

        //return-home
        if self.charge < 2 {
            let loading_station = robot_factory.get_random_station_location_with_type(StationType::LoadingDock);
            self.change_destination(loading_station);
        }
    }
}


#[cfg(test)]
mod tests {
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